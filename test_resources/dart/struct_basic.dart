class ViewData {
  const ViewData({
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

  const ViewData.createDefault()
      : deltaTime = 0.0,
        viewWidth = 0.0,
        viewHeight = 0.0,
        touchStartX = 0.0,
        touchStartY = 0.0,
        lastTouchX = 0.0,
        lastTouchY = 0.0,
        touchX = 0.0,
        touchY = 0.0;

  final double deltaTime;
  final double viewWidth;
  final double viewHeight;
  final double touchStartX;
  final double touchStartY;
  final double lastTouchX;
  final double lastTouchY;
  final double touchX;
  final double touchY;
}

class ViewDataBuffersFactory implements BuffersFactory<ViewData> {
  const ViewDataBuffersFactory();

  @override
  ViewData createDefault() => const ViewData.createDefault();
}
