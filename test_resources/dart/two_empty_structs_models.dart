class Struct1 {
  const Struct1();
}

class Struct1BuffersFactory implements BuffersFactory<Struct1> {
  const Struct1BuffersFactory();

  @override
  Struct1 createDefault() => const Struct1();
}

class Struct2 {
  const Struct2();
}

class Struct2BuffersFactory implements BuffersFactory<Struct2> {
  const Struct2BuffersFactory();

  @override
  Struct2 createDefault() => const Struct2();
}
