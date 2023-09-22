function(aq_require tgt)
    if(NOT TARGET ${tgt})
        add_subdirectory(${tgt})
    endif()
endfunction()

aq_require(acquire-core-libs)
aq_require(acquire-video-runtime)
