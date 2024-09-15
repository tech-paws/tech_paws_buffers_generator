class GenericType {
  const GenericType({
    required this.items,
    required this.table,
  });

  const GenericType.createDefault()
      : items = const <Test>[],
        table = const LinearTable<double, Test>.createDefault();

  final List<Test> items;
  final LinearTable<double, Test> table;
}

class GenericTypeBuffersFactory implements BuffersFactory<GenericType> {
  const GenericTypeBuffersFactory();

  @override
  GenericType createDefault() => const GenericType.createDefault();
}
