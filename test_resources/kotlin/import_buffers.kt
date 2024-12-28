// GENERATED, DO NOT EDIT
@file:Suppress("unused")

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*

data class Test(
    val value: Int,
) {
    companion object {
        fun createDefault(): Test = Test(
            value = 0,
        )

        fun readFromBuffers(reader: Long): Test {
            val value = Int.readFromBuffers(reader)

            return Test(
                value = value,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Int.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        value.writeToBuffers(writer)
    }
}
