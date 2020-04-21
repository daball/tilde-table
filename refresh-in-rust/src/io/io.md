# IO

Some brainstorming is required before going further.

Some of the ideas I had in mind:

- Accessing files.
- Accessing web resources.
- Accessing databases.

## Files

Opening a file path in the filesystem driver should return some type of
a read stream, or at least it should be easy to convert into a read stream.
This would enable things like seeking, reading an arbitrary amount of data,
manipulating the cursor location, and so forth.

Assuming that it contains text, this stream can then passed into a buffered text
adapter to buffer the reads.

Then, for instance, if you were to pass this buffered text reader into a CSV reader
for instance, you would then be able to inquire about the CSV data model and
read individual records and cells.

## Web resources

Opening a URI in the http driver should return some type of a read stream,
similar to the filesystem type but possibly with a different underlying handle
from the OS. Seeking may be possible, but may require advanced protocol features.
One ought to read all the data as soon as possible in case the network socket
closes, but bear in mind that it will be slower than a local filesystem read, so
async is king.

Assuming that it contains text, as it usually does, this stream can then be passed
into the same type of buffered text adapter.

Then, for instance, if it were to have JSON data or something, you would then be
able to inquire about the JSON data model and read parts of the tree.

## Database resources

Opening a database almost always involves a connection string. We can pass those
straight into an externally managed database driver, and we would expect to be
able to send queries over the network connection. Since the thing we are driving
is a relation itself, in the case of a database driver, we'll also need the query
to execute in order to get that data. Seeking is usually possible by using a
combination of LIMIT and OFFSET, so a stream interface could be implemented that
does something to that effect. A text adapter wouldn't be needed, but could be
used as a trivial record cursor for interfacing the user with the underlying data.
And any sort of filters could be arranged in a way to alter SQL queries.
