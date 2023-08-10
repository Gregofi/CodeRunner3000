-- Execute the users code
if arg[1] == nil then
    io.stderr:write("usage: evaluate lua_source")
    return
end

local users_code, err = loadfile(arg[1])

if users_code == nil then
    io.stderr:write(err)
    return
end

arg=nil
debug.debug=nil
debug.getfenv=nil
debug.getregistry=nil
dofile=nil
loadfile=nil
os.execute=nil
os.getenv=nil
os.remove=nil
os.rename=nil
os.tmpname=nil
package.loaded.io=io
package.loaded.package=nil
package=nil
require=nil

local res, error = pcall(users_code)
if res == false then
    io.stderr:write(error)
    return
end
