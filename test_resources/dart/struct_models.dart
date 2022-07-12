class Empty {
  const Empty();
}

class EmptyBuffersFactory implements BuffersFactory<Empty> {
  const EmptyBuffersFactory();

  @override
  Empty createDefault() => const Empty();
}

class ViewData {
  double deltaTime;
  double viewWidth;
  double viewHeight;
  double touchStartX;
  double touchStartY;
  double lastTouchX;
  double lastTouchY;
  double touchX;
  double touchY;

  ViewData({
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

  ViewData.createDefault()
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

class ViewDataBuffersFactory implements BuffersFactory<ViewData> {
  const ViewDataBuffersFactory();

  @override
  ViewData createDefault() => ViewData.createDefault();
}

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

class GenericType {
  List<Test> items;
  LinearTable<double, Test> table;

  GenericType({
    required this.items,
    required this.table,
  });

  GenericType.createDefault()
      : items = const ListBuffersFactory<Test>().createDefault(),
        table = const LinearTableBuffersFactory<double, Test>().createDefault();
}

class GenericTypeBuffersFactory implements BuffersFactory<GenericType> {
  const GenericTypeBuffersFactory();

  @override
  GenericType createDefault() => GenericType.createDefault();
}
