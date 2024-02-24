data class GenericType(
    val items: List<Test>,
    val table: LinearTable<Float, Test>,
) {
    companion object {
        fun createDefault(): GenericType = GenericType(
            items = listOf(),
            table = LinearTable.createDefault<Float, Test>(),
        )
    }
}
