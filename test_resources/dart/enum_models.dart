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
enum MyEnumWithoutPositionsValue {
  option1,
  option2,
  option3,
  option4,
}

class MyEnumWithoutPositions {
  MyEnumWithoutPositionsValue value = MyEnumWithoutPositionsValue.option_1;
  MyEnumWithoutPositionsOption1 option1 = MyEnumWithoutPositionsOption1.createDefault();
  MyEnumWithoutPositionsOption2 option2 = MyEnumWithoutPositionsOption2.createDefault();
  MyEnumWithoutPositionsOption3 option3 = const MyEnumWithoutPositionsOption3();
  MyEnumWithoutPositionsOption4 option4 = const MyEnumWithoutPositionsOption4();

  void toOption1(
    int v0,
  ) {
    value = MyEnumWithoutPositionsValue.option1;
    option1.v0 = v0;
  }

  void toOption2({
    required String name,
  }) {
    value = MyEnumWithoutPositionsValue.option2;
    option2.name = name,
  }

  void toOption3() => value = MyEnumWithoutPositionsValue.option3;

  void toOption4() => value = MyEnumWithoutPositionsValue.option4;
}

class MyEnumWithoutPositionsBuffersFactory implements BuffersFactory<MyEnumWithoutPositions> {
  const MyEnumWithoutPositionsBuffersFactory();

  @override
  MyEnumWithoutPositions createDefault() => MyEnumWithoutPositions();
}

class MyEnumWithoutPositionsOption1 {
  int v0;

  MyEnumWithoutPositionsOption1({
    required this.v0,
  });

  MyEnumWithoutPositionsOption1.createDefault()
      : v0 = ;
}

class MyEnumWithoutPositionsOption1BuffersFactory implements BuffersFactory<MyEnumWithoutPositionsOption1> {
  const MyEnumWithoutPositionsOption1BuffersFactory();

  @override
  MyEnumWithoutPositionsOption1 createDefault() => MyEnumWithoutPositionsOption1.createDefault();
}

class MyEnumWithoutPositionsOption2 {
  String name;

  MyEnumWithoutPositionsOption2({
    required this.name,
  });

  MyEnumWithoutPositionsOption2.createDefault()
      : name = const StringBuffersFactory().createDefault();
}

class MyEnumWithoutPositionsOption2BuffersFactory implements BuffersFactory<MyEnumWithoutPositionsOption2> {
  const MyEnumWithoutPositionsOption2BuffersFactory();

  @override
  MyEnumWithoutPositionsOption2 createDefault() => MyEnumWithoutPositionsOption2.createDefault();
}

class MyEnumWithoutPositionsOption3 {
  const MyEnumWithoutPositionsOption3();
}

class MyEnumWithoutPositionsOption3BuffersFactory implements BuffersFactory<MyEnumWithoutPositionsOption3> {
  const MyEnumWithoutPositionsOption3BuffersFactory();

  @override
  MyEnumWithoutPositionsOption3 createDefault() => const MyEnumWithoutPositionsOption3();
}

class MyEnumWithoutPositionsOption4 {
  const MyEnumWithoutPositionsOption4();
}

class MyEnumWithoutPositionsOption4BuffersFactory implements BuffersFactory<MyEnumWithoutPositionsOption4> {
  const MyEnumWithoutPositionsOption4BuffersFactory();

  @override
  MyEnumWithoutPositionsOption4 createDefault() => const MyEnumWithoutPositionsOption4();
}
