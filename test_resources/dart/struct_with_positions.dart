class Test {
  const Test({
    required this.touchY,
    required this.touchX,
    required this.touchStatus,
  });

  const Test.createDefault()
      : touchY = 0.0,
        touchX = 0.0,
        touchStatus = const TouchStatus.createDefault();

  final double touchY;
  final double touchX;
  final TouchStatus touchStatus;
}

class TestBuffersFactory implements BuffersFactory<Test> {
  const TestBuffersFactory();

  @override
  Test createDefault() => const Test.createDefault();
}
