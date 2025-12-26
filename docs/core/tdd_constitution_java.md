# 테스트 원칙 및 방법론 - Java

## 개요

이 문서는 **TDD(Test-Driven Development)**를 Java 환경에서 실천하기 위한 완전한 방법론을 제공한다.

### 이 문서를 숙지하면 할 수 있는 것

| 역량 | 설명 | 관련 섹션 |
|:-----|:-----|:-----|
| **Red-Green-Refactor 사이클 실천** | 실패하는 테스트 작성 → 통과하는 코드 구현 → 리팩토링의 반복 | Part 1, Section 1 |
| **효과적인 테스트 케이스 설계** | 경계값 분석, 동치 분할로 누락 없는 테스트 케이스 도출 | Part 1, Section 3 |
| **테스트 더블 활용** | Mock, Stub, Fake, Spy를 Mockito로 구현 | Part 1, Section 4 |
| **단위 테스트 작성** | Given-When-Then 패턴, JUnit 5 활용 | Part 2 |
| **통합 테스트 작성** | @SpringBootTest, Testcontainers 활용 | Part 3 |
| **E2E 테스트 설계** | REST Assured, Selenium 활용 | Part 4 |

### Red → Green → Refactor 달성 경로

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          TDD 사이클 달성 경로                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   🔴 RED (실패하는 테스트 작성)                                               │
│   ├─ Part 1, Section 3: 테스트 케이스 설계 기법으로 "무엇을 테스트할지" 결정     │
│   ├─ Part 2: 단위 테스트 작성 규칙으로 "어떻게 테스트할지" 결정                 │
│   └─ Part 3, 4: 통합/E2E 테스트로 "전체 흐름을 어떻게 검증할지" 결정            │
│                                                                             │
│                              ▼                                              │
│                                                                             │
│   🟢 GREEN (테스트를 통과하는 최소한의 코드 작성)                               │
│   ├─ Part 1, Section 1.2: "최소한의 코드"란 테스트만 통과하는 코드              │
│   ├─ Part 1, Section 4: Mockito로 외부 의존성 격리하며 구현                    │
│   └─ Part 2, Section 5.3: Given-When-Then 패턴으로 구현 결과 검증             │
│                                                                             │
│                              ▼                                              │
│                                                                             │
│   🔵 REFACTOR (코드 개선, 테스트는 계속 통과)                                  │
│   ├─ Part 1, Section 2: @BeforeEach로 중복 제거                              │
│   ├─ Part 2, Section 7: 테스트 우선순위로 중요한 것부터 개선                    │
│   └─ 모든 Part: 테스트가 통과하면 안전하게 리팩토링 가능                        │
│                                                                             │
│                              ▼                                              │
│                         (다음 기능으로 반복)                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

```

### TDD 숙련도 체크리스트

- [ ]  Red-Green-Refactor 사이클을 설명하고 실천할 수 있다
- [ ]  테스트를 먼저 작성하고, 그 테스트를 통과하는 코드를 구현할 수 있다
- [ ]  경계값 분석과 동치 분할로 테스트 케이스를 설계할 수 있다
- [ ]  Mock, Stub, Fake, Spy의 차이를 알고 Mockito로 적절히 구현할 수 있다
- [ ]  Given-When-Then 패턴으로 테스트를 구조화할 수 있다
- [ ]  @BeforeEach, @BeforeAll을 활용하여 테스트 간 중복을 제거할 수 있다
- [ ]  단위/통합/E2E 테스트의 차이를 알고 상황에 맞게 선택할 수 있다
- [ ]  테스트 피라미드 비율(Unit 70% : Integration 25% : E2E 5%)을 이해한다

---

## 목차

- [Part 1: TDD 방법론](#part-1-tdd-방법론)
- [Part 2: 단위 테스트 규칙](#part-2-단위-테스트-규칙)
- [Part 3: 통합 테스트 규칙](#part-3-통합-테스트-규칙)
- [Part 4: E2E 테스트 규칙](#part-4-e2e-테스트-규칙)

---

# Part 1: TDD 방법론

## 1. TDD란?

**TDD(Test-Driven Development, 테스트 주도 개발)**는 테스트를 먼저 작성하고, 그 테스트를 통과하는 코드를 작성하는 개발 방법론이다.

### 1.1 핵심 철학

> "동작하는 깔끔한 코드(Clean code that works)"를 목표로 한다.
> 
- **테스트가 개발을 이끈다**: 코드를 작성하기 전에 "이 코드가 무엇을 해야 하는가?"를 테스트로 먼저 정의
- **작은 단위로 반복**: 한 번에 하나의 기능만 테스트하고 구현
- **리팩토링 안전망**: 테스트가 있으면 코드를 안전하게 개선할 수 있음

### 1.2 Red-Green-Refactor 사이클

TDD의 핵심은 **3단계 사이클**을 반복하는 것이다:

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│    ┌─────────┐      ┌─────────┐      ┌───────────┐         │
│    │   Red   │ ───► │  Green  │ ───► │ Refactor  │ ──┐     │
│    │ (실패)  │      │ (통과)  │      │  (개선)   │   │     │
│    └─────────┘      └─────────┘      └───────────┘   │     │
│         ▲                                            │     │
│         └────────────────────────────────────────────┘     │
│                                                             │
└─────────────────────────────────────────────────────────────┘

```

| 단계 | 설명 | 예시 |
|:-----|:-----|:-----|
| **Red** | 실패하는 테스트 작성 | `testAdd()` 작성 → 실행 → 실패 (메서드가 없으니까) |
| **Green** | 테스트를 통과하는 최소한의 코드 작성 | `public int add(int a, int b) { return a + b; }` 구현 → 테스트 통과 |
| **Refactor** | 코드 개선 (테스트는 계속 통과해야 함) | 중복 제거, 네이밍 개선, 구조 정리 |

### 1.3 TDD 예시

```java
// 1단계: Red - 실패하는 테스트 작성
@Test
@DisplayName("10% 할인이 적용되어야 한다")
void calculateDiscount_TenPercent() {
    assertThat(calculator.calculateDiscount(1000, 10)).isEqualTo(900);
}
// → 실행하면 실패 (calculateDiscount 메서드가 없음)

// 2단계: Green - 테스트를 통과하는 최소한의 코드
public int calculateDiscount(int price, int percent) {
    return price - (price * percent / 100);
}
// → 테스트 통과

// 3단계: Refactor - 코드 개선
public int calculateDiscount(int price, int percent) {
    validatePercent(percent);
    int discountAmount = price * percent / 100;
    return price - discountAmount;
}

private void validatePercent(int percent) {
    if (percent < 0 || percent > 100) {
        throw new IllegalArgumentException("할인율은 0~100 사이여야 합니다");
    }
}
// → 테스트 여전히 통과, 코드는 더 견고해짐

```

---

## 2. TDD 적용 시나리오

### 2.1 현실적인 TDD

| 상황 | 적용 방법 |
|:-----|:---------|
| **새 기능 추가** | 테스트 먼저 작성 → 기능 구현 → 리팩토링 |
| **버그 수정** | 버그를 재현하는 테스트 작성 → 수정 → 테스트 통과 확인 |
| **기존 코드** | 현재 동작을 검증하는 테스트 작성 (Characterization Test) |

### 2.2 테스트 격리 (Test Isolation)

각 테스트는 **다른 테스트에 영향을 주거나 받지 않아야** 한다.

```java
// Bad: 테스트 간 상태 공유
class BadTest {
    private static int counter = 0;

    @Test
    void increment() {
        counter++;
        assertThat(counter).isEqualTo(1);  // 통과
    }

    @Test
    void incrementAgain() {
        counter++;
        assertThat(counter).isEqualTo(1);  // 실패! (counter가 이미 1이므로 2가 됨)
    }
}

// Good: 각 테스트가 독립적
class GoodTest {
    private int counter;

    @BeforeEach
    void setUp() {
        counter = 0;  // 매 테스트마다 초기화
    }

    @Test
    void increment() {
        counter++;
        assertThat(counter).isEqualTo(1);
    }

    @Test
    void incrementAgain() {
        counter++;
        assertThat(counter).isEqualTo(1);  // 통과
    }
}

```

---

## 3. 테스트 케이스 설계 기법

### 3.1 경계값 분석 (Boundary Value Analysis)

```java
// 예: 나이가 0~120 사이여야 유효한 경우
public boolean isValidAge(int age) {
    return age >= 0 && age <= 120;
}

// 경계값 테스트 케이스
@Test
@DisplayName("나이 경계값 테스트")
void isValidAge_BoundaryValues() {
    // 경계 안쪽 (유효)
    assertThat(validator.isValidAge(0)).isTrue();      // 최소 경계
    assertThat(validator.isValidAge(120)).isTrue();    // 최대 경계

    // 경계 바깥 (무효)
    assertThat(validator.isValidAge(-1)).isFalse();    // 최소 경계 -1
    assertThat(validator.isValidAge(121)).isFalse();   // 최대 경계 +1
}

```

### 3.2 동등 분할 (Equivalence Partitioning)

```java
// 예: 점수에 따른 등급 계산
public String getGrade(int score) {
    if (score >= 90) return "A";
    if (score >= 80) return "B";
    if (score >= 70) return "C";
    return "F";
}

// 동등 분할 테스트
@Test
@DisplayName("등급 동등 분할 테스트")
void getGrade_EquivalencePartitions() {
    assertThat(grader.getGrade(95)).isEqualTo("A");   // 90-100 파티션
    assertThat(grader.getGrade(85)).isEqualTo("B");   // 80-89 파티션
    assertThat(grader.getGrade(75)).isEqualTo("C");   // 70-79 파티션
    assertThat(grader.getGrade(50)).isEqualTo("F");   // 0-69 파티션
}

```

### 3.3 테스트 케이스 우선순위

| 우선순위 | 케이스 유형 | 이유 |
|:---------|:-----------|:-----|
| P0 | 정상 케이스 (Happy Path) | 기본 기능이 동작해야 함 |
| P1 | 경계값 케이스 | 버그가 자주 발생하는 지점 |
| P2 | 에러 케이스 | 예외 상황 처리 검증 |
| P3 | 극단적 케이스 | null 입력, 매우 큰 입력 등 |

---

## 4. 테스트 더블 (Test Double)

### 4.1 테스트 더블이란?

실제 객체 대신 **테스트용 가짜 객체**를 사용하여 외부 의존성을 격리한다.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        테스트 더블의 개념                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   실제 시스템                           테스트 환경                       │
│   ┌─────────────┐                      ┌─────────────┐                  │
│   │   내 코드   │                      │   내 코드   │                  │
│   └──────┬──────┘                      └──────┬──────┘                  │
│          │ 호출                               │ 호출                     │
│          ▼                                    ▼                         │
│   ┌─────────────┐                      ┌─────────────┐                  │
│   │  실제 DB    │  ─────────────►      │  Mock DB    │  ← 테스트 더블   │
│   │  실제 API   │      대체            │  Mock API   │                  │
│   └─────────────┘                      └─────────────┘                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘

```

### 4.2 Mockito를 활용한 테스트 더블

| 종류 | Mockito 구현 | 사용 시점 |
|:-----|:-------------|:---------|
| **Stub** | `when(...).thenReturn(...)` | 미리 정해진 값 반환 |
| **Mock** | `verify(...)` | 호출 여부/횟수 검증 |
| **Spy** | `@Spy`, `spy(...)` | 실제 동작 + 일부 stubbing |

### 4.3 Stub 예시

```java
@ExtendWith(MockitoExtension.class)
class UserServiceTest {

    @Mock
    private UserRepository userRepository;

    @InjectMocks
    private UserService userService;

    @Test
    @DisplayName("Stub: 사용자 조회 시 미리 정의된 값 반환")
    void findById_WithStub() {
        // Given: Stub 설정
        User stubUser = new User(1L, "홍길동");
        when(userRepository.findById(1L)).thenReturn(Optional.of(stubUser));

        // When
        User result = userService.findById(1L);

        // Then
        assertThat(result.getName()).isEqualTo("홍길동");
    }
}

```

### 4.4 Mock 예시

```java
@Test
@DisplayName("Mock: 알림 전송 메서드 호출 검증")
void completeOrder_SendsNotification() {
    // Given
    Order order = new Order(123L);

    // When
    orderService.completeOrder(order);

    // Then: 알림 메서드가 호출되었는지 검증
    verify(notificationService).sendNotification(
        eq("주문 123이 완료되었습니다.")
    );
    verify(notificationService, times(1)).sendNotification(anyString());
}

```

### 4.5 언제 테스트 더블을 사용하나?

| 상황 | 사용 여부 | 이유 |
|:-----|:---------|:-----|
| 외부 API 호출 | O | 네트워크 의존성 제거, 비용 절감 |
| 데이터베이스 접근 | O 또는 X | 단위 테스트는 Mock, 통합 테스트는 실제 DB |
| 시간 의존 로직 | O | Clock을 Mock하여 테스트 |
| 내부 유틸 클래스 | X | 실제 구현 사용 (과도한 Mock 지양) |

---

## 5. 테스트 유형 선택 가이드

### 5.1 선택 플로우차트

```
시작: "무엇을 테스트하려고 하는가?"
            │
            ▼
┌─────────────────────────────────────┐
│ Q1. 테스트 대상이 단일 클래스/메서드인가? │
└─────────────────────────────────────┘
         │                    │
        Yes                  No
         │                    │
         ▼                    ▼
┌────────────────┐   ┌─────────────────────────────────┐
│ Q2. 외부 의존성 │   │ Q3. UI/사용자 시나리오를 검증하나? │
│ 없이 테스트     │   └─────────────────────────────────┘
│ 가능한가?      │            │                    │
└────────────────┘           Yes                  No
    │        │                │                    │
   Yes      No                ▼                    ▼
    │        │        ┌────────────┐      ┌────────────────┐
    ▼        │        │  E2E 테스트 │      │   통합 테스트   │
┌───────────┐│        └────────────┘      └────────────────┘
│ 단위 테스트 ││
└───────────┘│
             ▼
   ┌────────────────────────────────────┐
   │ 의존성을 Mock으로 대체할 수 있는가? │
   └────────────────────────────────────┘
         │                    │
        Yes                  No
         │                    │
         ▼                    ▼
┌───────────────┐    ┌────────────────┐
│ 단위 테스트    │    │   통합 테스트   │
│ (Mockito 사용) │    │ (실제 의존성)  │
└───────────────┘    └────────────────┘

```

### 5.2 테스트 피라미드

```
                    ┌───────┐
                    │  E2E  │  ← 5% (핵심 시나리오만)
                   ─┴───────┴─
                  ┌───────────┐
                  │   통합    │  ← 25% (주요 워크플로우)
                 ─┴───────────┴─
                ┌───────────────┐
                │    단위       │  ← 70% (대부분의 테스트)
                └───────────────┘

```

---

# Part 2: 단위 테스트 규칙

## 6. 핵심 개념

### 6.1 @BeforeEach / @BeforeAll

테스트 실행 전에 필요한 데이터나 환경을 준비한다.

```java
@ExtendWith(MockitoExtension.class)
class CalculatorTest {

    private Calculator calculator;

    @BeforeAll
    static void setUpOnce() {
        // 전체 테스트 클래스에서 한 번만 실행
        System.out.println("테스트 클래스 시작");
    }

    @BeforeEach
    void setUp() {
        // 각 테스트 메서드 실행 전마다 실행
        calculator = new Calculator();
    }

    @AfterEach
    void tearDown() {
        // 각 테스트 메서드 실행 후마다 실행
    }

    @AfterAll
    static void tearDownOnce() {
        // 전체 테스트 클래스 종료 시 한 번만 실행
    }
}

```

| 어노테이션 | 실행 시점 | 사용 사례 |
|:-----------|:---------|:---------|
| `@BeforeAll` | 클래스당 한 번 (테스트 시작 전) | DB 연결, 무거운 리소스 초기화 |
| `@BeforeEach` | 각 테스트 메서드 전 | 테스트 객체 초기화, Mock 설정 |
| `@AfterEach` | 각 테스트 메서드 후 | 리소스 정리 |
| `@AfterAll` | 클래스당 한 번 (테스트 종료 후) | DB 연결 해제 |

### 6.2 테스트 마킹 (@Tag)

```java
// 테스트에 태그 적용
@Tag("slow")
@Test
void slowTest() { ... }

@Tag("api")
@Test
void apiTest() { ... }

@Tag("api")
@Tag("slow")
@Test
void slowApiTest() { ... }

```

```bash
# 특정 태그만 실행
./gradlew test --tests "*" -Ptags="api"

# 특정 태그 제외
./gradlew test --tests "*" -PexcludeTags="slow"

```

**build.gradle 설정:**

```groovy
test {
    useJUnitPlatform {
        includeTags project.findProperty('tags') ?: ''
        excludeTags project.findProperty('excludeTags') ?: ''
    }
}

```

### 6.3 테스트 커버리지 (JaCoCo)

```bash
# 커버리지 리포트 생성
./gradlew test jacocoTestReport

# 리포트 위치: build/reports/jacoco/test/html/index.html

```

**build.gradle 설정:**

```groovy
plugins {
    id 'jacoco'
}

jacocoTestReport {
    reports {
        xml.required = true
        html.required = true
    }
}

jacocoTestCoverageVerification {
    violationRules {
        rule {
            limit {
                minimum = 0.80  // 최소 80% 커버리지
            }
        }
    }
}

```

---

## 7. 테스트 작성 규칙

### 7.1 양방향 추적성 (Bidirectional Traceability)

**테스트 → 소스 코드 (테스트 파일에 명시)**

```java
/**
 * 대상: src/main/java/com/example/Calculator.java:45 - add()
 * 의도: 양수 두 개를 더했을 때 올바른 결과 반환 검증
 */
@Test
@DisplayName("양수 두 개 덧셈")
void add_PositiveNumbers() { ... }

```

**소스 코드 → 테스트 (소스 파일에 주석)**

```java
/**
 * 두 숫자를 더한 결과를 반환.
 *
 * @Test: src/test/java/.../CalculatorTest.java::add_PositiveNumbers
 */
public int add(int a, int b) {
    return a + b;
}

```

### 7.2 Given-When-Then 패턴

모든 테스트는 **Given-When-Then 패턴**을 따라 구조화한다.

```java
@Test
@DisplayName("유효한 ID로 사용자 조회 시 사용자 반환")
void findById_ValidId_ReturnsUser() {
    // Given: 테스트 데이터 및 환경 준비
    User expectedUser = new User(1L, "홍길동");
    when(userRepository.findById(1L)).thenReturn(Optional.of(expectedUser));

    // When: 테스트 대상 메서드 실행
    User result = userService.findById(1L);

    // Then: 결과 검증
    assertThat(result).isNotNull();
    assertThat(result.getName()).isEqualTo("홍길동");
}

```

| 섹션 | 역할 | 예시 |
|:-----|:-----|:-----|
| **Given** | 테스트에 필요한 데이터, 객체, 환경 준비 | Mock 설정, 테스트 데이터 생성 |
| **When** | 테스트 대상 메서드 실행 | `result = service.method(args)` |
| **Then** | 결과 검증 | `assertThat(result).isEqualTo(expected)` |

### 7.3 @DisplayName 필수

```java
// Bad: 메서드명만으로 의도 파악 어려움
@Test
void test1() { ... }

// Good: @DisplayName으로 의도 명확히
@Test
@DisplayName("존재하지 않는 ID로 조회 시 UserNotFoundException 발생")
void findById_InvalidId_ThrowsException() { ... }

```

---

## 8. 테스트 구조 규칙

### 8.1 네이밍 규칙

**클래스**: `{대상클래스}Test`

- `UserServiceTest`
- `OrderControllerTest`

**메서드**: `{메서드명}_{시나리오}_{예상결과}` 또는 `{메서드명}_{시나리오}`

- `findById_ValidId_ReturnsUser`
- `createOrder_InvalidInput_ThrowsException`
- `calculateDiscount_ZeroPercent`

### 8.2 @Nested를 활용한 그룹화

```java
@DisplayName("UserService 테스트")
class UserServiceTest {

    @Nested
    @DisplayName("findById 메서드")
    class FindById {

        @Test
        @DisplayName("유효한 ID로 조회 시 사용자 반환")
        void validId_ReturnsUser() { ... }

        @Test
        @DisplayName("존재하지 않는 ID로 조회 시 예외 발생")
        void invalidId_ThrowsException() { ... }
    }

    @Nested
    @DisplayName("createUser 메서드")
    class CreateUser {

        @Test
        @DisplayName("유효한 입력으로 사용자 생성")
        void validInput_CreatesUser() { ... }
    }
}

```

### 8.3 파라미터화 테스트

```java
@ParameterizedTest
@DisplayName("등급 계산 테스트")
@CsvSource({
    "95, A",
    "85, B",
    "75, C",
    "50, F"
})
void getGrade_VariousScores(int score, String expectedGrade) {
    assertThat(grader.getGrade(score)).isEqualTo(expectedGrade);
}

@ParameterizedTest
@DisplayName("경계값 테스트")
@ValueSource(ints = {-1, 0, 120, 121})
void isValidAge_BoundaryValues(int age) {
    // 테스트 로직
}

```

---

## 9. 단위 테스트 체크리스트

- [ ]  `@DisplayName`에 **의도**가 한글로 명시되어 있는가?
- [ ]  Javadoc에 **대상**(파일:라인 - 메서드명)이 명시되어 있는가?
- [ ]  **Given-When-Then 패턴**을 따르고 있는가?
- [ ]  재사용 설정은 **@BeforeEach**로 분리되어 있는가?
- [ ]  메서드명이 `{메서드명}_{시나리오}` 형식인가?
- [ ]  관련 테스트가 **@Nested**로 그룹화되어 있는가?

---

# Part 3: 통합 테스트 규칙

## 10. 통합 테스트란?

### 10.1 단위 테스트 vs 통합 테스트

| 구분 | 단위 테스트 | 통합 테스트 |
|:-----|:-----------|:-----------|
| **범위** | 클래스/메서드 하나 | 여러 컴포넌트 조합 |
| **의존성** | Mock 사용 | 실제 의존성 사용 |
| **어노테이션** | `@ExtendWith(MockitoExtension.class)` | `@SpringBootTest` |
| **속도** | 빠름 | 느림 |

### 10.2 Spring Boot 통합 테스트

```java
@SpringBootTest
@Transactional
class UserServiceIntegrationTest {

    @Autowired
    private UserService userService;

    @Autowired
    private UserRepository userRepository;

    @Test
    @DisplayName("통합: 사용자 생성 후 조회")
    void createAndFind_User() {
        // Given
        UserCreateRequest request = new UserCreateRequest("홍길동", "hong@example.com");

        // When
        User created = userService.createUser(request);
        User found = userService.findById(created.getId());

        // Then
        assertThat(found.getName()).isEqualTo("홍길동");
    }
}

```

---

## 11. 통합 테스트 작성 규칙

### 11.1 파일 구조

```
src/test/java/com/example/
├── unit/                          # 단위 테스트
│   ├── service/
│   │   └── UserServiceTest.java
│   └── controller/
│       └── UserControllerTest.java
│
└── integration/                   # 통합 테스트
    ├── UserServiceIntegrationTest.java
    └── OrderWorkflowIntegrationTest.java

```

### 11.2 네이밍 규칙

**클래스명**: `{대상}IntegrationTest`

- `UserServiceIntegrationTest`
- `OrderWorkflowIntegrationTest`

**메서드명**: `{시작}_{끝}_{시나리오}`

- `createUser_toFindUser_Success`
- `orderCreate_toPayment_ValidFlow`

### 11.3 @Tag 적용

```java
@SpringBootTest
@Tag("integration")
class UserServiceIntegrationTest {
    // ...
}

```

### 11.4 Testcontainers 활용

```java
@SpringBootTest
@Testcontainers
class UserRepositoryIntegrationTest {

    @Container
    static PostgreSQLContainer<?> postgres = new PostgreSQLContainer<>("postgres:15")
        .withDatabaseName("testdb")
        .withUsername("test")
        .withPassword("test");

    @DynamicPropertySource
    static void configureProperties(DynamicPropertyRegistry registry) {
        registry.add("spring.datasource.url", postgres::getJdbcUrl);
        registry.add("spring.datasource.username", postgres::getUsername);
        registry.add("spring.datasource.password", postgres::getPassword);
    }

    @Test
    void saveAndFind_User() {
        // 실제 PostgreSQL 컨테이너에서 테스트
    }
}

```

---

## 12. 통합 테스트 체크리스트

- [ ]  `@Tag("integration")` 태그가 붙어 있는가?
- [ ]  `@DisplayName`에 **통합 테스트 ID** (INT-XXX)가 명시되어 있는가?
- [ ]  클래스명이 `{대상}IntegrationTest` 형식인가?
- [ ]  `@Transactional`로 테스트 간 데이터 격리가 되어 있는가?
- [ ]  `integration/` 패키지에 위치해 있는가?

---

# Part 4: E2E 테스트 규칙

## 13. E2E 테스트란?

**사용자 관점에서 전체 시스템**을 검증하는 테스트다.

### 13.1 Java E2E 테스트 도구

| 도구 | 용도 | 특징 |
|:-----|:-----|:-----|
| **REST Assured** | API E2E 테스트 | HTTP 요청/응답 검증 |
| **Selenium** | UI E2E 테스트 | 브라우저 자동화 |
| **Testcontainers** | 환경 구성 | Docker 기반 의존성 |

---

## 14. REST Assured를 활용한 API E2E 테스트

```java
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@Tag("e2e")
class UserApiE2ETest {

    @LocalServerPort
    private int port;

    @BeforeEach
    void setUp() {
        RestAssured.port = port;
    }

    @Test
    @DisplayName("E2E: 사용자 생성 → 조회 → 수정 → 삭제")
    void userCrudFlow() {
        // 1. 사용자 생성
        String userId = given()
            .contentType(ContentType.JSON)
            .body("""
                {
                    "name": "홍길동",
                    "email": "hong@example.com"
                }
                """)
        .when()
            .post("/api/users")
        .then()
            .statusCode(201)
            .extract()
            .path("id");

        // 2. 사용자 조회
        given()
        .when()
            .get("/api/users/{id}", userId)
        .then()
            .statusCode(200)
            .body("name", equalTo("홍길동"));

        // 3. 사용자 수정
        given()
            .contentType(ContentType.JSON)
            .body("""
                {
                    "name": "김철수"
                }
                """)
        .when()
            .put("/api/users/{id}", userId)
        .then()
            .statusCode(200);

        // 4. 사용자 삭제
        given()
        .when()
            .delete("/api/users/{id}", userId)
        .then()
            .statusCode(204);
    }
}

```

---

## 15. E2E 테스트 작성 규칙

### 15.1 파일 구조

```
src/test/java/com/example/
├── unit/
├── integration/
└── e2e/                           # E2E 테스트
    ├── api/
    │   └── UserApiE2ETest.java
    └── ui/
        └── LoginFlowE2ETest.java

```

### 15.2 네이밍 규칙

**클래스명**: `{대상}E2ETest`

- `UserApiE2ETest`
- `LoginFlowE2ETest`

**메서드명**: `{시나리오}Flow` 또는 `{시나리오}Journey`

- `userCrudFlow`
- `loginToOrderJourney`

### 15.3 Docstring 형식

```java
/**
 * E2E 테스트 ID: E2E-001
 * 시나리오: 사용자 CRUD 전체 흐름
 *
 * 검증 단계:
 * 1. 사용자 생성 (POST /api/users)
 * 2. 사용자 조회 (GET /api/users/{id})
 * 3. 사용자 수정 (PUT /api/users/{id})
 * 4. 사용자 삭제 (DELETE /api/users/{id})
 *
 * 예상 결과: 모든 CRUD 작업이 정상 수행됨
 */
@Test
@DisplayName("E2E: 사용자 CRUD 전체 흐름")
void userCrudFlow() { ... }

```

---

## 16. 테스트 실행

```bash
# 단위 테스트만 실행
./gradlew test -PexcludeTags="integration,e2e"

# 통합 테스트만 실행
./gradlew test -Ptags="integration"

# E2E 테스트만 실행
./gradlew test -Ptags="e2e"

# 전체 테스트 실행
./gradlew test

```

---

## 17. E2E 테스트 체크리스트

- [ ]  `@Tag("e2e")` 태그가 붙어 있는가?
- [ ]  Javadoc에 **E2E 테스트 ID** (E2E-XXX)가 명시되어 있는가?
- [ ]  Javadoc에 **검증 단계**가 순서대로 명시되어 있는가?
- [ ]  `e2e/` 패키지에 위치해 있는가?
- [ ]  테스트가 **독립적으로 실행 가능**한가?

---

## 18. 의존성 설정 (build.gradle)

```groovy
dependencies {
    // JUnit 5
    testImplementation 'org.junit.jupiter:junit-jupiter:5.10.0'

    // Mockito
    testImplementation 'org.mockito:mockito-core:5.5.0'
    testImplementation 'org.mockito:mockito-junit-jupiter:5.5.0'

    // AssertJ
    testImplementation 'org.assertj:assertj-core:3.24.2'

    // Spring Boot Test (통합 테스트)
    testImplementation 'org.springframework.boot:spring-boot-starter-test'

    // Testcontainers (통합 테스트)
    testImplementation 'org.testcontainers:junit-jupiter:1.19.0'
    testImplementation 'org.testcontainers:postgresql:1.19.0'

    // REST Assured (E2E 테스트)
    testImplementation 'io.rest-assured:rest-assured:5.3.2'
}

test {
    useJUnitPlatform()
}

```

---