sealed interface MyEnum {
    data object Idle : MyEnum

    data class Move(
        val y: Double,
        val x: Double,
    ) : MyEnum

    data class Update(
        val p1: String,
        val p2: Float,
        val p3: Int,
        val p8: Double,
    ) : MyEnum

    companion object {
        fun createDefault(): MyEnum = Idle

        fun readFromBuffers(reader: Long): MyEnum {
            return when (val case = UInt.readFromBuffers(reader)) {
                3U -> Idle
                1U -> {
                    val y = Double.readFromBuffers(reader)
                    val x = Double.readFromBuffers(reader)

                    Move(
                        y = y,
                        x = x,
                    )
                }
                2U -> {
                    val p1 = String.readFromBuffers(reader)
                    val p2 = Float.readFromBuffers(reader)
                    val p3 = Int.readFromBuffers(reader)
                    val p8 = Double.readFromBuffers(reader)

                    Update(
                        p1 = p1,
                        p2 = p2,
                        p3 = p3,
                        p8 = p8,
                    )
                }
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
            Idle -> {
                3UL.writeToBuffers(writer)
            }
            is Move -> {
                1UL.writeToBuffers(writer)
                y.writeToBuffers(writer)
                x.writeToBuffers(writer)
            }
            is Update -> {
                2UL.writeToBuffers(writer)
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
                p3.writeToBuffers(writer)
                p8.writeToBuffers(writer)
            }
        }
    }
}
