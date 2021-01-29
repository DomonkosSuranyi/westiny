use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};
use amethyst::core::ecs::{Write, Read};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use amethyst::core::Transform;

#[derive(SystemDesc)]
#[system_desc(name(ServerNetworkSystemDesc))]
pub struct ServerNetworkSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ServerNetworkSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'s> System<'s> for ServerNetworkSystem {
    type SystemData = (
        Write<'s, TransportResource>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, net_event_ch): Self::SystemData) {
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, _) => {
                    let initial_pos = crate::network::ConnectionPackage { initial_trans: Transform::default()};
                    let msg = bincode::serialize(&initial_pos).unwrap(); // TODO result
                    log::info!("Message received [{}], sending response...", addr);
                    net.send_with_requirements(*addr, &msg, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick);
                }
                _ => log::info!("Network event: {:?}", event)

            }
        }
    }
}