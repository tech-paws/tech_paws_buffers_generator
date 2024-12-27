sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle

        fun readFromBuffers(reader: Long): MyEnum {
            val case = UInt.readFromBuffers(reader)

            return when (case) {
                3U -> MyEnumIdle.readFromBuffers(reader)
                1U -> MyEnumMove.readFromBuffers(reader)
                2U -> MyEnumUpdate.readFromBuffers(reader)
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
    val y: Double,
    val x: Double,
) : MyEnum {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumMove {
            val y = Double.readFromBuffers(reader)
            val x = Double.readFromBuffers(reader)

            return MyEnumMove(
                y = y,
                x = x,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        y.writeToBuffers(writer)
        x.writeToBuffers(writer)
    }
}

data class MyEnumUpdate(
    val p1: String,
    val p2: Float,
    val p3: Int,
    val p8: Double,
) : MyEnum {
    companion object {
        fun readFromBuffers(reader: Long): MyEnumUpdate {
            val p1 = String.readFromBuffers(reader)
            val p2 = Float.readFromBuffers(reader)
            val p3 = Int.readFromBuffers(reader)
            val p8 = Double.readFromBuffers(reader)

            return MyEnumUpdate(
                p1 = p1,
                p2 = p2,
                p3 = p3,
                p8 = p8,
            )
        }
    }

    override fun writeToBuffers(writer: Long) {
        p1.writeToBuffers(writer)
        p2.writeToBuffers(writer)
        p3.writeToBuffers(writer)
        p8.writeToBuffers(writer)
    }
}
