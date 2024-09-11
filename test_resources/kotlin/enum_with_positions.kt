sealed interface MyEnum {
    companion object {
        fun createDefault(): MyEnum = MyEnumIdle
    }
}

object MyEnumIdle : MyEnum

data class MyEnumMove(
    val y: Double,
    val x: Double,
) : MyEnum

data class MyEnumUpdate(
    val p1: String,
    val p2: Float,
    val p3: Int,
    val p8: Double,
) : MyEnum
