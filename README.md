# file-system-monit
file-system-monit - rust lang

이 프로젝트는 Rust로 작성된 간단한 파일 시스템 모니터링 도구입니다. 지정된 디렉토리의 파일 시스템 변경 사항을 실시간으로 감지하고 로깅합니다.

## 기능

- 지정된 디렉토리와 그 하위 디렉토리의 파일 시스템 변경 감지
- 변경 이벤트를 syslog에 로깅

## 필요 조건

- Rust 프로그래밍 언어
- Cargo 패키지 관리자

## 사용 방법

다음 명령어로 프로그램을 실행합니다:
cargo run -- /path/to/watch

여기서 `/path/to/watch`는 모니터링하고자 하는 디렉토리 경로입니다.

## 예제 코드

다음은 `src/main.rs` 파일의 주요 부분입니다:

```rust
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter, warn};

fn main() {
    let path = std::env::args().nth(1).expect("Argument 1 needs to be a path");

    // syslog 설정
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "fs-watcher".into(),
        pid: 0,
    };
    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Warn))
        .expect("could not register logger");

    println!("Watching {}", path);
    if let Err(e) = watch(path) {
        println!("Error: {:?}", e);
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => log_event(event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn log_event(event: notify::Event) {
    warn!("Change occurred: {:?}", event);
}
```

라이선스
이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 LICENSE 파일을 참조하세요.

이 README.md 파일은 프로젝트의 개요, 설치 방법, 사용 방법, 그리고 주요 코드 예제를 포함하고 있습니다. 필요에 따라 추가 정보나 설명을 더할 수 있습니다. 예를 들어, 의존성 패키지 목록, 더 자세한 설정 방법, 문제 해결 팁 등을 추가할 수 있습니다.
