# the-middle-of-namui-and-react

I think namui can reduce the amount of rendering and updating, like react!

리액트는 상태를 Component가 들고 있게 한다. 상태는 처음에 바로 초기화가 안되어도 상관 없다. JS가 뭐 그렇지.

우리는 컴포넌트를 어떻게 생성할 것인가?
컴포넌트를 생성하는 함수를 시스템에, 렌더 반환값으로 제공해줄 수 있다.
stateful한 경우를 만들지 않는다면 어떻게 될까?

stateful하지 않다는 것은 무엇일까?
어떤 것이 stateful하다는 것은 결국 그것이 state를 들고 있다는 뜻이다.
직접 소유하고 있는 state를 직접 유지&관리해야만 원하는 바를 수행할 수 있으면 stateful이다.

stateless하려면 어떻게 해야할까?
state를 직접 유지&관리 하면 안된다.
근데 특정 컴포넌트가 특정 state를 가져야할 때는?
아예 3자가 state를 가질 순 있다. 그럼 문제는 그 state에 접근해야하는데, 어떻게 접근하는가?

> TODO 위 질문에 대한 답 써보기

React의 Render는 새로 컴포넌트를 만드는지, 아니면 기존의 것을 재활용하는지 구분이 되지 않는다.
일종의 선언형 프로그래밍이다. 최종적으로는 꼭 하나 존재하게 만든다.

초기값을 부모가 제공해주는게 맞을까?
props까지는 부모가 제공해줄 수 있다고 본다.

컴포넌트 생성자: Fn(Props) -> Component
렌더: Fn(&mut Component, Props) -> RenderingTree

이렇게 하면 되지 않을까?
