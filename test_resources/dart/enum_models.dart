enum MyEnumValue {
  idle,
  move,
  update,
}

class MyEnum {
  MyEnumValue value = MyEnumValue.idle;
  MyEnumIdle idle = const MyEnumIdle();
  MyEnumMove move = MyEnumMove.createDefault();
  MyEnumUpdate update = MyEnumUpdate.createDefault();

  void toIdle() => value = MyEnumValue.idle;

  void toMove({
    required double x,
    required double y,
  }) {
    value = MyEnumValue.move;
    move.x = x,
    move.y = y,
  }

  void toUpdate(
    double v0,
    double v1,
    String v2,
  ) {
    value = MyEnumValue.update;
    update.v0 = v0;
    update.v1 = v1;
    update.v2 = v2;
  }
}

class MyEnumBuffersFactory implements BuffersFactory<MyEnum> {
  const MyEnumBuffersFactory();

  @override
  MyEnum createDefault() => MyEnum();
}

class MyEnumIdle {
  const MyEnumIdle();
}

class MyEnumIdleBuffersFactory implements BuffersFactory<MyEnumIdle> {
  const MyEnumIdleBuffersFactory();

  @override
  MyEnumIdle createDefault() => const MyEnumIdle();
}

class MyEnumMove {
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

  @override
  MyEnumMove createDefault() => MyEnumMove.createDefault();
}

class MyEnumUpdate {
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

  @override
  MyEnumUpdate createDefault() => MyEnumUpdate.createDefault();
}
