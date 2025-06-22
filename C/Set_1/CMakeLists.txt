cmake_minimum_required(VERSION 3.10)

project(Task_1)

add_executable(Task_1 main.c Utils.c)

target_compile_options(Task_1 PRIVATE -Wall -g)

include_directories(${CMAKE_SOURCE_DIR}/Utils.h)

target_sources(${PROJECT_NAME}
        PRIVATE
        

        PUBLIC
        Utils.h
        Utils.c
        main.c
        )