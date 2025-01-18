// identity: 노드에 대한 새로운 무작위 키 페어를 생성한다.
// PeerId: PeerId구조체는 노드의 공개 키로부터 피어 ID를 생성하는 메서드를 포함하고 있다.
use libp2p::{identity, PeerId};

#[tokio::main]
async fn main() {
    // ED25519 타입의 키 페어를 생성한다. 키 페어는 하나의 비밀 키와 하나의 공개 키로 구성된다.
    let new_key = identity::Keypair::generate_ed25519();
    // 키 페어의 공개 키로부터 피어 ID를 생성한다. libp2p 에서는 공개 키를 직접 사용하는 것이 아니라
    // 공개 키의 해시값을 사용해서 피어를 식별한다.
    let new_peer_id = PeerId::from(new_key.public());

    println!("New peer id: {:?}", new_peer_id);
}