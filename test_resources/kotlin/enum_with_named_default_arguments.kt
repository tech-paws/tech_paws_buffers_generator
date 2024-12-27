sealed interface MyEnumWithNamedArguments {
    companion object {
        fun createDefault(): MyEnumWithNamedArguments = MyEnumWithNamedArgumentsOption1(
            name = "",
            value = 0f,
            bytes = listOf(),
        )

        fun readFromBuffers(reader: Long): MyEnumWithNamedArguments {
            val case = UInt.readFromBuffers(reader)

            return when (case) {
                0U -> MyEnumWithNamedArgumentsOption1.readFromBuffers(reader)
                1U -> MyEnumWithNamedArgumentsOption2.readFromBuffers(reader)
                2U -> MyEnumWithNamedArgumentsOption3.readFromBuffers(reader)
                3U -> MyEnumWithNamedArgumentsOption4.readFromBuffers(reader)
                else -> throw IllegalArgumentException("Invalid enum value: $case")
            }
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long)
}

data class MyEnumWithNamedArgumentsOption1(
    val name: String,
    val value: Float,
    val bytes: List<UByte>,
) : MyEnumWithNamedArguments {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithNamedArgumentsOption1 {
            val name = String.readFromBuffers(reader)
            val value = Float.readFromBuffers(reader)
            val bytes = readFromBuffersList(reader) {
                UByte.readFromBuffers(reader)
            }

            return MyEnumWithNamedArgumentsOption1(
                name = name,
                value = value,
                bytes = bytes,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        name.writeToBuffers(writer)
        value.writeToBuffers(writer)
        writeToBuffersList(writer, bytes) { bytesItem ->
            bytesItem.writeToBuffers(writer)
        }
    }
}

data class MyEnumWithNamedArgumentsOption2(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithNamedArguments {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithNamedArgumentsOption2 {
            val p0 = ULong.readFromBuffers(reader)
            val p1 = ULong.readFromBuffers(reader)
            val p2 = ULong.readFromBuffers(reader)

            return MyEnumWithNamedArgumentsOption2(
                p0 = p0,
                p1 = p1,
                p2 = p2,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        p0.writeToBuffers(writer)
        p1.writeToBuffers(writer)
        p2.writeToBuffers(writer)
    }
}

data object MyEnumWithNamedArgumentsOption3 : MyEnumWithNamedArguments {
    fun readFromBuffers(reader: Long): MyEnumWithNamedArgumentsOption3 {
        return MyEnumWithNamedArgumentsOption3
    }

    override fun writeToBuffers(writer: Long) {
    }
}

data object MyEnumWithNamedArgumentsOption4 : MyEnumWithNamedArguments {
    fun readFromBuffers(reader: Long): MyEnumWithNamedArgumentsOption4 {
        return MyEnumWithNamedArgumentsOption4
    }

    override fun writeToBuffers(writer: Long) {
    }
}
