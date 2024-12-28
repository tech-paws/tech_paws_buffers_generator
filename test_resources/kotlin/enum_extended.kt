sealed interface MyEnum {
    data object Idle : MyEnum

    data class Move(
        val x: Double,
        val y: Double,
    ) : MyEnum

    data class Update(
        val p1: Double,
        val p2: Double,
        val p4: String,
    ) : MyEnum

    companion object {
        fun createDefault(): MyEnum = Idle

        fun readFromBuffers(reader: Long): MyEnum {
            return when (val case = UInt.readFromBuffers(reader)) {
                1U -> Idle
                2U -> {
                    val x = Double.readFromBuffers(reader)
                    val y = Double.readFromBuffers(reader)

                    Move(
                        x = x,
                        y = y,
                    )
                }
                3U -> {
                    val p1 = Double.readFromBuffers(reader)
                    val p2 = Double.readFromBuffers(reader)
                    val p4 = String.readFromBuffers(reader)

                    Update(
                        p1 = p1,
                        p2 = p2,
                        p4 = p4,
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
            Idle -> {}
            is Move -> {
                x.writeToBuffers(writer)
                y.writeToBuffers(writer)
            }
            is Update -> {
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
                p4.writeToBuffers(writer)
            }
        }
    }
}

sealed interface MyEnumWithoutPositions {
    data class Option1(
        val p0: ULong,
    ) : MyEnumWithoutPositions

    data class Option2(
        val name: String,
    ) : MyEnumWithoutPositions

    data object Option3 : MyEnumWithoutPositions

    data object Option4 : MyEnumWithoutPositions

    companion object {
        fun createDefault(): MyEnumWithoutPositions = Option1(
            0UL,
        )

        fun readFromBuffers(reader: Long): MyEnumWithoutPositions {
            return when (val case = UInt.readFromBuffers(reader)) {
                0U -> {
                    val p0 = ULong.readFromBuffers(reader)

                    Option1(
                        p0 = p0,
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
                p0.writeToBuffers(writer)
            }
            is Option2 -> {
                name.writeToBuffers(writer)
            }
            Option3 -> {}
            Option4 -> {}
        }
    }
}

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
                p0.writeToBuffers(writer)
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
            }
            is Option2 -> {
                name.writeToBuffers(writer)
            }
            Option3 -> {}
            Option4 -> {}
        }
    }
}

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
                name.writeToBuffers(writer)
                value.writeToBuffers(writer)
                writeToBuffersList(writer, bytes) { bytesItem ->
                    bytesItem.writeToBuffers(writer)
                }
            }
            is Option2 -> {
                p0.writeToBuffers(writer)
                p1.writeToBuffers(writer)
                p2.writeToBuffers(writer)
            }
            Option3 -> {}
            Option4 -> {}
        }
    }
}
