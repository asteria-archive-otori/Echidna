CMake_Minimum_Required(VERSION 3.1.0)

Project(echidna VERSION 0.0 LANGUAGES CXX)

Set(CMAKE_CXX_STANDARD 11)
Set(CMAKE_CXX_STANDARD_REQUIRED ON)

Set(CMAKE_AUTOMOC ON)
Set(CMAKE_AUTORCC ON)
Set(CMAKE_AUTOUIC ON)



Find_Package(Qt5 COMPONENTS Widgets REQUIRED)

add_subdirectory(src)

file(GLOB SOURCES src/*.cpp)
file(GLOB HEADERS src/*.h)

Add_Executable(echidna
        ${SOURCES}      
        ${HEADERS}
        src/echidna.ui
        src/resources.qrc
)



Target_Link_Libraries(echidna Qt5::Widgets)


