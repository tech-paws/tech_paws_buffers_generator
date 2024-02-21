struct Empty {
    static func createDefault() -> Empty {
        return Empty()
    }
}

struct ViewData {
    var deltaTime: Float
    var viewWidth: Float
    var viewHeight: Float
    var touchStartX: Float
    var touchStartY: Float
    var lastTouchX: Float
    var lastTouchY: Float
    var touchX: Float
    var touchY: Float

    static func createDefault() -> ViewData {
        return ViewData(
            deltaTime: 0,
            viewWidth: 0,
            viewHeight: 0,
            touchStartX: 0,
            touchStartY: 0,
            lastTouchX: 0,
            lastTouchY: 0,
            touchX: 0,
            touchY: 0
        )
    }
}

struct Test {
    var touchX: Float
    var touchY: Float
    var touchStatus: TouchStatus

    static func createDefault() -> ViewData {
        return Test(
            touchX: Float,
            touchY: Float,
            touchStatus: TouchStatus.createDefault()
        )
    }
}

struct GenericType {
    var items: [Test]
    var table: LinearTable<Float, Test>

    static func createDefault() -> GenericType {
        return GenericType(
            items: [],
            table: LinearTable<Float, Test>.createDefault()
        )
    }
}
