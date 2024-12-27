sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle

        fun readFromBuffers(reader: Long): MyEnum {
            val case = UInt.readFromBuffers(reader)

            return when (case) {
                1U -> MyEnumIdle.readFromBuffers(reader)
                2U -> MyEnumMove.readFromBuffers(reader)
                3U -> MyEnumUpdate.readFromBuffers(reader)
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

data object MyEnumIdle : MyEnum {
    fun readFromBuffers(reader: Long): MyEnumIdle {
        return MyEnumIdle
    }

    override fun writeToBuffers(writer: Long) {
    }
}

data class MyEnumMove(
    val x: Double,
    val y: Double,
) : MyEnum {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumMove {
            val x = Double.readFromBuffers(reader)
            val y = Double.readFromBuffers(reader)

            return MyEnumMove(
                x = x,
                y = y,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        x.writeToBuffers(writer)
        y.writeToBuffers(writer)
    }
}

data class MyEnumUpdate(
    val p1: Double,
    val p2: Double,
    val p4: String,
) : MyEnum {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumUpdate {
            val p1 = Double.readFromBuffers(reader)
            val p2 = Double.readFromBuffers(reader)
            val p4 = String.readFromBuffers(reader)

            return MyEnumUpdate(
                p1 = p1,
                p2 = p2,
                p4 = p4,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        p1.writeToBuffers(writer)
        p2.writeToBuffers(writer)
        p4.writeToBuffers(writer)
    }
}

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
