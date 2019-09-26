Consider this scenario: You copied a bunch of files from a dir to another a while ago. But now you decided you don't want those files/directories that are scattered in there. If you're lucky you can just ctrl+z it, but if not you'll have to do something to select the files based on dir1 to remove them in dir2. This program automates this. You specify a template dir, and a target dir, and whatever files/dirs exist in the template will be removed in the target.

New: Now it supports using a zip file as a template. Just use a path to a zip file instead of a directory.

For instance:

dir1:
- 1.txt
- 3.txt
- dir1
- dir3

dir2:
- 1.txt
- 2.txt
- 3.txt
- dir1
- dir2
- dir3

Lets remove whatever is inside dir1 in dir2:

`schoon /home/me/dir1 /home/me/dir2`

Result:

dir1 (unchanged):
- 1.txt
- 3.txt
- dir1
- dir3

dir2:
- 2.txt
- dir2