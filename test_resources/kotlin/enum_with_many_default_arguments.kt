sealed interface MyEnumWithManyArguments {
    data class Option1(
        val p0: ULong,
        val p1: ULong,
        val p2: ULong,
    ) : MyEnumWithManyArguments

    data class Option2(
        val name: String,
    ) : MyEnumWithManyArguments

    data object Option3 : MyEnumWithManyArguments

    data object Option4 : MyEnumWithManyArguments

    companion object {
        fun createDefault(): MyEnumWithManyArguments = Option1(
            0UL,
            0UL,
            0UL,
        )

        fun readFromBuffers(reader: Long): MyEnumWithManyArguments {
            return when (val case = UInt.readFromBuffers(reader)) {
                0U -> {
                    val p0 = ULong.readFromBuffers(reader)
                    val p1 = ULong.readFromBuffers(reader)
                    val p2 = ULong.readFromBuffers(reader)

                    Option1(
                        p0 = p0,
                        p1 = p1,
                        p2 = p2,
                    )
                }
                1U -> {
                    val name = String.readFromBuffers(reader)

                    Option2(
                        name = name,
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
                0UL.writeToBuffers(writer)
                p0.writeToBuffers(writer)
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
            }
            is Option2 -> {
                1UL.writeToBuffers(writer)
                name.writeToBuffers(writer)
            }
            Option3 -> {
                2UL.writeToBuffers(writer)
            }
            Option4 -> {
                3UL.writeToBuffers(writer)
            }
        }
    }
}
