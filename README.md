# Duck Duck Go Image Download

__Work In Progress!__

This is a small utility written in Rust to find and download
images matching given keywords using the Duck Duck Go search
engine.

It is generally fit for purpose except that:
* No validation is done on missing or invalid images.
* Only the first 100 search results are used currently
* There are outstanding TODO items (see below)

__PRIOR ART__

I heavily referenced the [duckduckgo-images-api](https://github.com/KshitijMhatre/duckduckgo-images-api) NPM
package when writing this, which itself was inspired by [a Python package](https://github.com/deepanprabhu/duckduckgo-images-api).

Thanks to both authors for their efforts.

__NOTE:__

I'm writing this as an exercise in learning Rust
so that should give you an idea as to the quality to expect!

__TODO:__

* Filter on image type if option provided
* Download specified number of images if provided
* Remove invalid images
* Better error handling
* Tests!