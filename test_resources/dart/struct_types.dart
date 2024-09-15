class BasicTypesModel {
  const BasicTypesModel({
    required this.byte,
    required this.someInteger,
    required this.someLong,
    required this.someUnsigedInteger,
    required this.someUnsigedLong,
    required this.someFloatNumber,
    required this.someDoubleNumber,
    required this.someBool,
    required this.someString,
    required this.vector,
    required this.generic,
    required this.custom,
    required this.optionalString,
    required this.optionalF32,
  });

  const BasicTypesModel.createDefault()
      : byte = 0,
        someInteger = 0,
        someLong = 0,
        someUnsigedInteger = 0,
        someUnsigedLong = 0,
        someFloatNumber = 0.0,
        someDoubleNumber = 0.0,
        someBool = false,
        someString = "",
        vector = const <String>[],
        generic = const LinearTable<double, Test>.createDefault(),
        custom = const MyModel.createDefault(),
        optionalString = null,
        optionalF32 = null;

  final int byte;
  final int someInteger;
  final int someLong;
  final int someUnsigedInteger;
  final int someUnsigedLong;
  final double someFloatNumber;
  final double someDoubleNumber;
  final bool someBool;
  final String someString;
  final List<String> vector;
  final LinearTable<double, Test> generic;
  final MyModel custom;
  final String? optionalString;
  final double? optionalF32;
}

class BasicTypesModelBuffersFactory implements BuffersFactory<BasicTypesModel> {
  const BasicTypesModelBuffersFactory();

  @override
  BasicTypesModel createDefault() => const BasicTypesModel.createDefault();
}
