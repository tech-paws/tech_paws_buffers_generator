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
    required double field0,
    required double field1,
    required String field2,
  }) =>
      MyEnumUpdate(
        field0: field0,
        field1: field1,
        field2: field2,
      );
}

class MyEnumIdle implements MyEnum {
  const MyEnumIdle();
}

class MyEnumMove implements MyEnum {
  double x;
  double y;

  MyEnumMove({
    required this.x,
    required this.y,
  });
}

class MyEnumUpdate implements MyEnum {
  double field0;
  double field1;
  String field2;

  MyEnumUpdate({
    required this.field0,
    required this.field1,
    required this.field2,
  });
}
