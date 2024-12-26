class Empty() {
    companion object {
        fun createDefault(): Empty = Empty()

        fun readFromBuffers(reader: Long): Empty {
            return Empty()
        }

        fun skipInBuffers(reader: Long, count: Int) {
        }
    }

    fun writeToBuffers(writer: Long) {
    }
}
