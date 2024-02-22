struct Test {
    var touchX: Float
    var touchY: Float
    var touchStatus: TouchStatus

    static func createDefault() -> Test {
        return Test(
            touchX: 0,
            touchY: 0,
            touchStatus: TouchStatus.createDefault()
        )
    }
}
