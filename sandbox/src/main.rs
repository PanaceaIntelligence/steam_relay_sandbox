use steamworks::{Client, NetworkingConfigValue, NetworkingSockets};

fn main() {
    // 1. Steam API 초기화 (실제 실행 시 Steam 클라이언트가 켜져 있어야 함)
    let (client, single) = Client::init().expect("Steam을 초기화할 수 없습니다.");
    
    // 2. Networking Sockets 인터페이스 가져오기
    let networking_sockets = client.networking_sockets();

    // 3. Relay(SDR) 네트워크 접근 강제 활성화 (선택 사항이지만 추천)
    // 이 작업을 통해 연결 시 발생할 수 있는 초기 지연을 줄입니다.
    client.networking_utils().init_relay_network_access();

    println!("Steam Relay 준비 완료!");

    // 메인 루프 (콜백 처리를 위해 필요)
    loop {
        single.run_callbacks();
        // 여기에 게임 로직 및 데이터 수신 로직 추가
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}