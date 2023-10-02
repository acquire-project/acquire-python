cmake_minimum_required(VERSION 3.5)
project(acquire-libs)

function(aq_require tgt)
    if(NOT TARGET ${tgt})
        add_subdirectory(${tgt})
    endif()
endfunction()

aq_require(acquire-core-libs)
aq_require(acquire-video-runtime)
