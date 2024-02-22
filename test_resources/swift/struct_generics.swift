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
