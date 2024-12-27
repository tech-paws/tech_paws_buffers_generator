sealed interface MyEnumWithoutPositions {
    companion object {
        fun createDefault(): MyEnumWithoutPositions = MyEnumWithoutPositionsOption1(
            0UL,
        )

        fun readFromBuffers(reader: Long): MyEnumWithoutPositions {
            val case = UInt.readFromBuffers(reader)

            return when (case) {
                0U -> MyEnumWithoutPositionsOption1.readFromBuffers(reader)
                1U -> MyEnumWithoutPositionsOption2.readFromBuffers(reader)
                2U -> MyEnumWithoutPositionsOption3.readFromBuffers(reader)
                3U -> MyEnumWithoutPositionsOption4.readFromBuffers(reader)
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

data class MyEnumWithoutPositionsOption1(
    val p0: ULong,
) : MyEnumWithoutPositions {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithoutPositionsOption1 {
            val p0 = ULong.readFromBuffers(reader)

            return MyEnumWithoutPositionsOption1(
                p0 = p0,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        p0.writeToBuffers(writer)
    }
}

data class MyEnumWithoutPositionsOption2(
    val name: String,
) : MyEnumWithoutPositions {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumWithoutPositionsOption2 {
            val name = String.readFromBuffers(reader)

            return MyEnumWithoutPositionsOption2(
                name = name,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        name.writeToBuffers(writer)
    }
}

data object MyEnumWithoutPositionsOption3 : MyEnumWithoutPositions {
    fun readFromBuffers(reader: Long): MyEnumWithoutPositionsOption3 {
        return MyEnumWithoutPositionsOption3
    }

    override fun writeToBuffers(writer: Long) {
    }
}

data object MyEnumWithoutPositionsOption4 : MyEnumWithoutPositions {
    fun readFromBuffers(reader: Long): MyEnumWithoutPositionsOption4 {
        return MyEnumWithoutPositionsOption4
    }

    override fun writeToBuffers(writer: Long) {
    }
}
