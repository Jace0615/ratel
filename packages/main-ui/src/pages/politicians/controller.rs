use std::str::FromStr;

use bdk::prelude::*;
use by_types::QueryResponse;
use dto::*;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub politicians: Resource<QueryResponse<AssemblyMemberSummary>>,
    pub sort: Signal<Option<AssemblyMemberSorter>>,
    pub order: Signal<SortOrder>,
    pub stance: Signal<Option<CryptoStance>>,
    pub party: Signal<Option<String>>,
}

impl Controller {
    pub fn new(_lang: Language) -> std::result::Result<Self, RenderError> {
        let size = 1000;
        let sort = use_signal(|| None);
        let order = use_signal(|| SortOrder::Ascending);
        let stance = use_signal(|| None);
        let party = use_signal(|| None);

        let politicians = use_server_future(move || {
            let cli = AssemblyMember::get_client(&crate::config::get().main_api_endpoint);
            let sort = sort();
            let stance = stance();
            let order = order();
            let party = party();

            async move {
                let mut q = AssemblyMemberQuery::new(size).with_order(order);

                if let Some(party) = party {
                    q = q.with_party(party);
                }

                if let Some(sort) = sort {
                    q = q.with_sort(sort);
                }

                if let Some(stance) = stance {
                    q = q.with_stance(stance);
                }

                cli.query(q).await.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            politicians,
            sort,
            stance,
            order,
            party,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn set_sort(&mut self, sort: AssemblyMemberSorter) {
        if self.sort() == Some(sort) {
            self.order.set(match self.order() {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            });
        } else {
            self.order.set(SortOrder::Ascending);
            self.sort.set(Some(sort));
        }
    }

    pub fn set_stance(&mut self, stance: String) {
        let stance = CryptoStance::from_str(&stance).unwrap_or_default();
        self.stance.set(match stance {
            CryptoStance::None => None,
            s => Some(s),
        });
    }

    pub fn set_party(&mut self, party: String) {
        let party = Party::from_str(&party).unwrap_or_default();
        self.party.set(match party {
            Party::None => None,
            p => Some(p.translate(&Language::Ko).to_string()),
        });
    }
}
