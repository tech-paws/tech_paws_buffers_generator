sealed interface MyEnumWithNamedArguments {
    data class Option1(
        val name: String,
        val value: Float,
        val bytes: List<UByte>,
    ) : MyEnumWithNamedArguments

    data class Option2(
        val p0: ULong,
        val p1: ULong,
        val p2: ULong,
    ) : MyEnumWithNamedArguments

    data object Option3 : MyEnumWithNamedArguments

    data object Option4 : MyEnumWithNamedArguments

    companion object {
        fun createDefault(): MyEnumWithNamedArguments = Option1(
            name = "",
            value = 0f,
            bytes = listOf(),
        )

        fun readFromBuffers(reader: Long): MyEnumWithNamedArguments {
            return when (val case = UInt.readFromBuffers(reader)) {
                0U -> {
                    val name = String.readFromBuffers(reader)
                    val value = Float.readFromBuffers(reader)
                    val bytes = readFromBuffersList(reader) {
                        UByte.readFromBuffers(reader)
                    }

                    Option1(
                        name = name,
                        value = value,
                        bytes = bytes,
                    )
                }
                1U -> {
                    val p0 = ULong.readFromBuffers(reader)
                    val p1 = ULong.readFromBuffers(reader)
                    val p2 = ULong.readFromBuffers(reader)

                    Option2(
                        p0 = p0,
                        p1 = p1,
                        p2 = p2,
                    )
                }
                2U -> Option3
                3U -> Option4
                else -> throw IllegalArgumentException("Invalid enum value: $case")
            }
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        when (this) {
            is Option1 -> {
                0U.writeToBuffers(writer)
                name.writeToBuffers(writer)
                value.writeToBuffers(writer)
                writeToBuffersList(writer, bytes) { bytesItem ->
                    bytesItem.writeToBuffers(writer)
                }
            }
            is Option2 -> {
                1U.writeToBuffers(writer)
                p0.writeToBuffers(writer)
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
            }
            Option3 -> {
                2U.writeToBuffers(writer)
            }
            Option4 -> {
                3U.writeToBuffers(writer)
            }
        }
    }
}
