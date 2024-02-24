sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle
    }
}

object MyEnumIdle : MyEnum

data class MyEnumMove(
    val x: Double,
    val y: Double,
) : MyEnum

data class MyEnumUpdate(
    val p0: Double,
    val p1: Double,
    val p2: String,
) : MyEnum

sealed interface MyEnumWithoutPositions {
    companion object {
        fun createDefault(): MyEnumWithoutPositions = MyEnumWithoutPositionsOption1(
            0UL,
        )
    }
}

data class MyEnumWithoutPositionsOption1(
    val p0: ULong,
) : MyEnumWithoutPositions

data class MyEnumWithoutPositionsOption2(
    val name: String,
) : MyEnumWithoutPositions

object MyEnumWithoutPositionsOption3 : MyEnumWithoutPositions

object MyEnumWithoutPositionsOption4 : MyEnumWithoutPositions

sealed interface MyEnumWithManyArguments {
    companion object {
        fun createDefault(): MyEnumWithManyArguments = MyEnumWithManyArgumentsOption1(
            0UL,
            0UL,
            0UL,
        )
    }
}

data class MyEnumWithManyArgumentsOption1(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithManyArguments

data class MyEnumWithManyArgumentsOption2(
    val name: String,
) : MyEnumWithManyArguments

object MyEnumWithManyArgumentsOption3 : MyEnumWithManyArguments

object MyEnumWithManyArgumentsOption4 : MyEnumWithManyArguments

sealed interface MyEnumWithNamedArguments {
    companion object {
        fun createDefault(): MyEnumWithNamedArguments = MyEnumWithNamedArgumentsOption1(
            name = "",
            value = 0f,
            bytes = listOf(),
        )
    }
}

data class MyEnumWithNamedArgumentsOption1(
    val name: String,
    val value: Float,
    val bytes: List<UByte>,
) : MyEnumWithNamedArguments

data class MyEnumWithNamedArgumentsOption2(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithNamedArguments

object MyEnumWithNamedArgumentsOption3 : MyEnumWithNamedArguments

object MyEnumWithNamedArgumentsOption4 : MyEnumWithNamedArguments
