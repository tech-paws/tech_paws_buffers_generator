data class GenericType(
    val items: List<Test>,
    val table: LinearTable<Float, Test>,
) {
    companion object {
        fun createDefault(): GenericType = GenericType(
            items = listOf(),
            table = LinearTable.createDefault<Float, Test>(),
        )

        fun readFromBuffers(reader: Long): GenericType {
            val items = readFromBuffersList(reader) {
                Test.readFromBuffers(reader)
            }
            val table = LinearTable<Float, Test>.readFromBuffers(reader)

            return GenericType(
                items = items,
                table = table,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                readFromBuffersList(reader) {
                    Test.readFromBuffers(reader)
                }
                LinearTable<Float, Test>.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        writeToBuffersList(writer, items) { itemsItem ->
            itemsItem.writeToBuffers(writer)
        }
        table.writeToBuffers(writer)
    }
}
