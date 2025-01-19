// 스웜은 libp2p의 노드와 관련된 네트워크 관리자 컴포넌트이다.
// 노드 사이의 데이터 스트림을 교환하는 데 사용된다.
use libp2p::{
    autonat,
    futures::StreamExt,
    identity, identify, // 피어 식별
    PeerId,
    mdns,
    multiaddr::Protocol, // 멀티주소 프로토콜
    noise, // 암호화 프로토콜
    tcp, yamux, Multiaddr, SwarmBuilder, // 주소 및 스웜 구성
    swarm::{Swarm, SwarmEvent, NetworkBehaviour},
    ping
};
use std::error::Error;
use std::time::Duration;

#[derive(NetworkBehaviour)]
struct Behaviour {
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}
/**
 * libp2p는 p2p 애플리케이션 개발을 가능하게 하는 프로토콜, 명세, 라이브러리의
 * 모듈러 시스템이다. libp2p의 핵심 아키텍처 컴포넌트는 전송, 신원, 보안, 피어 발견, 
 * 피어 라우팅, 콘텐트 라우팅과 메시징이다.
 */
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
        // Ping과 mDNS를 결합한 네트워크 동작 생성
        .with_behaviour(|key| {
            Ok(Behaviour {
                ping: ping::Behaviour::new(ping::Config::new()),
                mdns: mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id())?,
            })
        })?
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

    // 로컬 노드에서 원격 노드로 나가는 연결을 나타낸다.
    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer_multiaddr)?;
        println!("Dialed remote peer: {:?}", remote_peer);
    }
    /*
    swarm.listen_on(
        Multiaddr::empty()
            .with(Protocol::Ip4(Ipv4Addr::UNSPECIFIED))
            .with(Protocol::Tcp(opt.listen_port)),
    )?;
     */
    

    // cargo run --bin iter3 /ip4/127.0.0.1/tcp/----- -> 첫음 실행한 노드에서 나오는 포트번호 입력
    loop {
        match swarm.select_next_some().await {
            // 새로운 리스닝 주소가 할당되면 출력
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            // mDNS를 통해 새로운 피어를 발견했을 때
            SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer, addr) in peers {
                    println!("discovered peer: {} with addr: {}", peer, addr);
                }
            }
            // mDNS에서 피어가 사라졌을 때
            SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                for (peer, addr) in peers {
                    println!("peer expired: {} at addr: {}", peer, addr);
                }
            }
            // Ping 이벤트 발생 시
            SwarmEvent::Behaviour(BehaviourEvent::Ping(event)) => {
                match event {
                    ping::Event {
                        peer,
                        result: Result::Ok(rtt),
                        ..
                    } => {
                        println!("Ping: rtt to  {} is {} ms", peer.to_base58(), rtt.as_millis());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Timeout),
                        ..
                    } => {
                        println!("ping: timeout to {}", peer.to_base58());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Unsupported),
                        ..
                    } => {
                        println!("ping: {} does not support ping protocol", peer.to_base58());
                    }
                    ping::Event {
                        peer,
                        result: Result::Err(ping::Failure::Other { error }),
                        ..
                    } => {
                        println!("ping: ping::Failure with {}: {error}", peer.to_base58());
                    }
                    _ => {
                        println!("Ping Err");
                    }
                }
            }
            _ => {}
        }
    }
}