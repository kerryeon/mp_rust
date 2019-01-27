# Machine Pseudo-Code
```
Main:
    Print! "hello world"
```
`Machine Pseudo-Code` (이하 `MP`)는 알고리즘을 더욱 쉽게 기술하기 위한
`범용 프로그래밍 언어`입니다.
[기존의 프로젝트 (Python)](https://github.com/kerryeon/mp)을 직접적으로 계승하였으며,
기존의 단점을 개선하고 가독성에 최대한 집중하였습니다.
많은 경우 `Rust`, `Python`과 `ML`에 영향을 받았습니다.

## 언어의 특징
* 범용 프로그래밍 언어: 어떠한 프로그램도 개발 가능하도록 설계되었습니다.
* 인터프리터 언어: 소스 코드를 환경의 제약없이 실행할 수 있습니다.
* 함수형 프로그래밍 지원: 함수명과 입출력에 집중하며 프로그래밍할 수 있습니다.
* 제네릭 프로그래밍 지원: 많은 경우 타입에 집중하지 않고 프로그래밍할 수 있습니다.
* 프로토타입 기반 프로그래밍 지원: 함수는 선언을 통해 객체가 될 수 있고, 상속이 가능합니다.
* 소유권 기반 프로그래밍 지원: `Rust`의 소유자 개념을 도입했습니다.
    * 인터프리터임에도 불구하고, `GC` 없이 메모리를 안전하게 관리할 수 있습니다.

## 예시
1. 팩토리얼 계산
```
Factorial[]: x -> z
    z: match! x <= 1
        yes: 1
        no:
            Factorial! (x-1) -> y
            x * y

Main:
    5 -> ?:
        Factorial!
        Print!

```
2. `LeNet`
```
Using[]:
    Deep learning:
        platform: PyTorch  # Tensorflow, CNTK, ...

My Convolution: Convolution 2D
    kernel size: 5, 5
    padding: 2, 2

My Pooling: Max Pooling 2D
    kernel size: 2, 2

LeNet: x -> x
    My Convolution!
        channel: 1, 32
    My Pooling!
    ReLU!
    My Convolution!
        channel: 32, 64
    My Pooling!
    ReLU!
    Transform!
        dim: ?, $(64 * 7 * 7)
    Dense!
        dim in: ?, $(64 * 7 * 7)
        dim out: ?, 1024
    ReLU!
    Dense!
        dim in: ?, 1024
        dim out: ?, 10
    LogSoftmax!

Main:
    model: LeNet
    ...

```
