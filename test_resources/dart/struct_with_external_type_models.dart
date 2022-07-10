class Test {
  double touchX;
  double touchY;
  TouchStatus touchStatus;

  Test({
    required this.touchX,
    required this.touchY,
    required this.touchStatus,
  });

  Test.createDefault()
      : touchX = 0.0,
        touchY = 0.0,
        touchStatus = const TouchStatusBuffersFactory().createDefault();
}

class TestBuffersFactory implements BuffersFactory<Test> {
  const TestBuffersFactory();

  @override
  Test createDefault() => Test.createDefault();
}
