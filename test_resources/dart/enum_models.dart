enum MyEnumValue {
  idle,
  move,
  update,
}

class MyEnumUnion {
  MyEnumValue value = MyEnumValue.idle;
  MyEnumIdle idle = const MyEnumIdle();
  MyEnumMove move = MyEnumMove.createDefault();
  MyEnumUpdate update = MyEnumUpdate.createDefault();
}

abstract class MyEnum {
  static const idle = MyEnumIdle();

  static MyEnumMove move({
    required double x,
    required double y,
  }) =>
      MyEnumMove(
        x: x,
        y: y,
      );

  static MyEnumUpdate update({
    required double v0,
    required double v1,
    required String v2,
  }) =>
      MyEnumUpdate(
        v0: v0,
        v1: v1,
        v2: v2,
      );
}

class MyEnumIdle implements MyEnum {
  const MyEnumIdle();
}

class MyEnumIdleBuffersFactory implements BuffersFactory<MyEnumIdle> {
  const MyEnumIdleBuffersFactory();

  MyEnumIdle createDefault() => const MyEnumIdle();
}

class MyEnumMove implements MyEnum {
  double x;
  double y;

  MyEnumMove({
    required this.x,
    required this.y,
  });

  MyEnumMove.createDefault()
      : x = 0.0,
        y = 0.0;
}

class MyEnumMoveBuffersFactory implements BuffersFactory<MyEnumMove> {
  const MyEnumMoveBuffersFactory();

  MyEnumMove createDefault() => MyEnumMove.createDefault();
}

class MyEnumUpdate implements MyEnum {
  double v0;
  double v1;
  String v2;

  MyEnumUpdate({
    required this.v0,
    required this.v1,
    required this.v2,
  });

  MyEnumUpdate.createDefault()
      : v0 = 0.0,
        v1 = 0.0,
        v2 = const StringBuffersFactory().createDefault();
}

class MyEnumUpdateBuffersFactory implements BuffersFactory<MyEnumUpdate> {
  const MyEnumUpdateBuffersFactory();

  MyEnumUpdate createDefault() => MyEnumUpdate.createDefault();
}
