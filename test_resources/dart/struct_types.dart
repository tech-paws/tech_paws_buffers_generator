class BasicTypesModel {
  const BasicTypesModel({
    required this.byte,
    required this.someInteger,
    required this.someLong,
    required this.someUnsignedInteger,
    required this.someUnsignedLong,
    required this.someFloatNumber,
    required this.someDoubleNumber,
    required this.someBool,
    required this.someString,
    required this.vector,
    required this.generic,
    required this.custom,
    required this.optionalString,
    required this.optionalListString,
    required this.listOptionalString,
    required this.listListString,
    required this.optionalF32,
  });

  const BasicTypesModel.createDefault()
      : byte = 0,
        someInteger = 0,
        someLong = 0,
        someUnsignedInteger = 0,
        someUnsignedLong = 0,
        someFloatNumber = 0.0,
        someDoubleNumber = 0.0,
        someBool = false,
        someString = "",
        vector = const <String>[],
        generic = const LinearTable<double, Test>.createDefault(),
        custom = const MyModel.createDefault(),
        optionalString = null,
        optionalListString = null,
        listOptionalString = const <String?>[],
        listListString = const <List<String>>[],
        optionalF32 = null;

  final int byte;
  final int someInteger;
  final int someLong;
  final int someUnsignedInteger;
  final int someUnsignedLong;
  final double someFloatNumber;
  final double someDoubleNumber;
  final bool someBool;
  final String someString;
  final List<String> vector;
  final LinearTable<double, Test> generic;
  final MyModel custom;
  final String? optionalString;
  final List<String>? optionalListString;
  final List<String?> listOptionalString;
  final List<List<String>> listListString;
  final double? optionalF32;
}

class BasicTypesModelBuffersFactory implements BuffersFactory<BasicTypesModel> {
  const BasicTypesModelBuffersFactory();

  @override
  BasicTypesModel createDefault() => const BasicTypesModel.createDefault();
}
