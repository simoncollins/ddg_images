# Duck Duck Go Image Download

__Work In Progress!__

This is a small utility written in Rust to find and download
images matching given keywords using the Duck Duck Go search
engine.

Things are mostly working but currently only uses the first
search term you specify and only writes out the first
result and to the working directory.

__PRIOR ART__

I heavily referenced the [duckduckgo-images-api](https://github.com/KshitijMhatre/duckduckgo-images-api) NPM
package when writing this, which itself was inspired by [a Python package](https://github.com/deepanprabhu/duckduckgo-images-api).

Thanks to both authors for their efforts.

__NOTE:__

I'm currently writing this as an exercise in learning Rust
so that should give you an idea as to the quality to expect!

__TODO:__

* Use all search terms
* Download more than the first image!
* Use base directory option if provided
* Download images to sub-directories named after each keyword
* Filter on image type if option provided
* Download specified number of images if provided
* Remove invalid images
* Better error handling
* Tests!