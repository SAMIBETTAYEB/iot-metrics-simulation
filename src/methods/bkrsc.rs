use std::env;

use lazy_static::lazy_static;

use crate::{
    CommunicationOverheadType, CommunicationType, EnergyConsumptionType, EnergyType,
    ExchangeCostType, ExchangeType, MetricsType, StateCostType, InvolvedDevicesCount, InvolvedExchangesCount,
};

lazy_static! {
    static ref EPSB: f32 = env::var("EPSB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref EPRB: f32 = env::var("EPRB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref SENT_MESSAGE_SIZE: u32 = env::var("SENT_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
    static ref RECEIVED_MESSAGE_SIZE: u32 = env::var("RECEIVED_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
}

pub fn get_metrics(
    number_of_nodes: u32,
    number_of_gateway_members: u32,
    number_of_neighbors: u32,
) -> MetricsType {
    let metrics = MetricsType {
        energy: EnergyType {
            compromised: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_nodes,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::All),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0f32,
                        received: 0f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 0,
                        received: 0,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
            draining: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_gateway_members,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::GatewayMembers),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0f32,
                        received: 0f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 0,
                        received: 0,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
            leaving: EnergyConsumptionType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_neighbors,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::Neighbors),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: 0f32,
                        received: 0f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *EPSB,
                        received: *EPRB,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
        },
        communication: CommunicationType {
            compromised: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_nodes,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::All),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 0,
                        received: 0,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
            draining: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_gateway_members,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::GatewayMembers),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 0,
                        received: 0,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
            leaving: CommunicationOverheadType {
                constrained: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: 1,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: number_of_gateway_members,
                    involved_devices: InvolvedDevicesCount::GatewayMembers,
                },
                gateway: StateCostType {
                    exchange: ExchangeType {
                        sent: 1,
                        received: number_of_neighbors,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::Neighbors),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 1,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
                left: StateCostType {
                    exchange: ExchangeType {
                        sent: 0,
                        received: 0,
                        messages: (InvolvedExchangesCount::SameAsDefined, InvolvedExchangesCount::SameAsDefined),
                    },
                    exchange_cost: ExchangeCostType {
                        sent: *SENT_MESSAGE_SIZE as f32,
                        received: *RECEIVED_MESSAGE_SIZE as f32,
                    },
                    number_of_involved_devices: 0,
                    involved_devices: InvolvedDevicesCount::SameAsDefined,
                },
            },
        },
    };
    metrics
}
