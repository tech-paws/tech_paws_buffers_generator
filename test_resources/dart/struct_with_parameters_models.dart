class Test {
  double deltaTime;
  double viewWidth;
  double viewHeight;
  double touchStartX;
  double touchStartY;
  double lastTouchX;
  double lastTouchY;
  double touchX;
  double touchY;

  Test({
    required this.deltaTime,
    required this.viewWidth,
    required this.viewHeight,
    required this.touchStartX,
    required this.touchStartY,
    required this.lastTouchX,
    required this.lastTouchY,
    required this.touchX,
    required this.touchY,
  });

  Test.createDefault()
      : deltaTime = 0.0,
        viewWidth = 0.0,
        viewHeight = 0.0,
        touchStartX = 0.0,
        touchStartY = 0.0,
        lastTouchX = 0.0,
        lastTouchY = 0.0,
        touchX = 0.0,
        touchY = 0.0;
}

class TestBuffersFactory implements BuffersFactory<Test> {
  const TestBuffersFactory();

  @override
  Test createDefault() => Test.createDefault();
}
