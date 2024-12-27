sealed interface MyEnumWithManyArguments {
    companion object {
        fun createDefault(): MyEnumWithManyArguments = MyEnumWithManyArgumentsOption1(
            0UL,
            0UL,
            0UL,
        )

        fun readFromBuffers(reader: Long): MyEnumWithManyArguments {
            val case = UInt.readFromBuffers(reader)

            return when (case) {
                0U -> MyEnumWithManyArgumentsOption1.readFromBuffers(reader)
                1U -> MyEnumWithManyArgumentsOption2.readFromBuffers(reader)
                2U -> MyEnumWithManyArgumentsOption3.readFromBuffers(reader)
                3U -> MyEnumWithManyArgumentsOption4.readFromBuffers(reader)
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

data class MyEnumWithManyArgumentsOption1(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithManyArguments {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithManyArgumentsOption1 {
            val p0 = ULong.readFromBuffers(reader)
            val p1 = ULong.readFromBuffers(reader)
            val p2 = ULong.readFromBuffers(reader)

            return MyEnumWithManyArgumentsOption1(
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

data class MyEnumWithManyArgumentsOption2(
    val name: String,
) : MyEnumWithManyArguments {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithManyArgumentsOption2 {
            val name = String.readFromBuffers(reader)

            return MyEnumWithManyArgumentsOption2(
                name = name,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        name.writeToBuffers(writer)
    }
}

data object MyEnumWithManyArgumentsOption3 : MyEnumWithManyArguments {
    fun readFromBuffers(reader: Long): MyEnumWithManyArgumentsOption3 {
        return MyEnumWithManyArgumentsOption3
    }

    override fun writeToBuffers(writer: Long) {
    }
}

data object MyEnumWithManyArgumentsOption4 : MyEnumWithManyArguments {
    fun readFromBuffers(reader: Long): MyEnumWithManyArgumentsOption4 {
        return MyEnumWithManyArgumentsOption4
    }

    override fun writeToBuffers(writer: Long) {
    }
}
