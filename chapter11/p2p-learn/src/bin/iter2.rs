use libp2p::swarm::behaviour;
// 스웜은 libp2p의 노드와 관련된 네트워크 관리자 컴포넌트이다.
use libp2p::swarm::{Swarm, SwarmEvent, dummy};
// 노드 사이의 데이터 스트림을 교환하는 데 사용된다.
use libp2p::{identity, PeerId}; // noise::{Keypair, NoiseConfig, X25519Spec}
use libp2p::{
    autonat,
    futures::StreamExt,
    identify, // 피어 식별
    multiaddr::Protocol, // 멀티주소 프로토콜
    noise, // 암호화 프로토콜
    tcp, yamux, Multiaddr, SwarmBuilder, // 주소 및 스웜 구성
};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("local peer id is: {:?}", new_peer_id);

    // development_transport -> libp2p::SwarmBuilder 사용으로 대체됨

    // p2p 네트워크 스웜 구성
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            // 하나의 네트워크 연결을 여러 개의 가상 채널로 나누는 프로토콜
            // 예: 하나의 채널로 파일을 전송, 다른 채널로 메시지를 주고 받고, 또 다른 채널로 상태 정보 교환
            yamux::Config::default 
        )?
        .with_quic()
        .with_dns()?
        // 스웜과 연결하기 위해 더미 네트워크 동작 생성 
        .with_behaviour(|key| dummy::Behaviour)?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    /**
     * with_dns()
     * 피어들이 도메인 이름으로 서로를 찾을 수 있게 하고
     * 동적 IP 환경에서도 안정적인 연결을 유지할 수 있게 하며
     * 특히 클라우드나 서버리스 환경에서 피어 디스커버리를 용이하게 한다.
     */

    // 모든 네트워크 인터페이스에서 임의의 포트로 리스닝
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?);

    /*
    swarm.listen_on(
        Multiaddr::empty()
            .with(Protocol::Ip4(Ipv4Addr::UNSPECIFIED))
            .with(Protocol::Tcp(opt.listen_port)),
    )?;
     */
    
    loop {
        match swarm.select_next_some().await {
            // 새로운 리스닝 주소가 할당되면 출력
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            _ => {}
        }
    }
}