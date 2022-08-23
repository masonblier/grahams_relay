
import bpy
import math
import mathutils
import os

# override print output to in-gui console
def println(data):
    for window in bpy.context.window_manager.windows:
        screen = window.screen
        for area in screen.areas:
            if area.type == 'CONSOLE':
                override = {'window': window, 'screen': screen, 'area': area}
                bpy.ops.console.scrollback_append(override, text=str(data), type="OUTPUT")

def rotate_scene_x(degrees):
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        obj.select_set(True)
    bpy.ops.transform.rotate(value=math.radians(degrees),center_override=(0,0,0), orient_axis='X')
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        obj.select_set(False)

def build_ron():
    colliderstr = ""
    doorstr = ""
    interactablestr = ""
    propstr = ""

    for obj in bpy.data.objects:
#        if obj.type != "MESH":
#            continue

        name = bpy.path.clean_name(obj.name)

        if obj.name.startswith("Collider"):
            ctype = obj.name.split(".")[1]
            matrix_world = obj.matrix_world
            t = matrix_world.to_translation()
            r = matrix_world.to_quaternion()
            s = matrix_world.to_scale()
            colliderstr +=("    WorldCollider(\n")
            colliderstr +=("      shape: \"" + str(ctype) + "\",\n")
            colliderstr +=("      translation: Vec3(" + str(t[0])+","+str(t[2])+","+str(-t[1]) + "),\n")
            colliderstr +=("      rotation: Quat("+str(r[1])+","+str(r[3])+","+str(-r[2])+","+str(r[0])+","+"),\n")
            colliderstr +=("      scale: Vec3(" + str(abs(s[0]))+","+str(abs(s[2]))+","+str(abs(s[1])) + "),\n")
            colliderstr +=("    ),\n")

        if obj.name.startswith("Door"):
            dprop = obj.name.split(".")[1]
            dname = obj.name.split(".")[2]
            matrix_world = obj.matrix_world
            t = matrix_world.to_translation()
            r = matrix_world.to_quaternion()
            s = matrix_world.to_scale()
            doorstr +=("    WorldDoor(\n")
            doorstr +=("      prop: \"" + str(dprop) + "\",\n")
            doorstr +=("      name: \"" + str(dname) + "\",\n")
            doorstr +=("      translation: Vec3(" + str(t[0])+","+str(t[2])+","+str(-t[1]) + "),\n")
            doorstr +=("      rotation: Quat("+str(r[1])+","+str(r[3])+","+str(-r[2])+","+str(r[0])+","+"),\n")
            doorstr +=("      scale: Vec3(" + str(abs(s[0]))+","+str(abs(s[2]))+","+str(abs(s[1])) + "),\n")
            doorstr +=("    ),\n")

        if obj.name.startswith("Interactable"):
            ctype = obj.name.split(".")[1]
            matrix_world = obj.matrix_world
            t = matrix_world.to_translation()
            r = matrix_world.to_quaternion()
            s = matrix_world.to_scale()
            interaction = "None"
            if "interaction" in obj:
                interaction_type = "\""+obj["interaction"]+"\""
                interaction_actions = ""
                if "interaction_actions" in obj:
                    interaction_actions = ""+obj["interaction_actions"]+""
                interaction = "Some(WorldInteraction(\n"
                interaction += "        interaction: "+interaction_type+",\n"
                interaction += "        actions: ["+interaction_actions+"],\n"
                interaction += "      ))"
            interactablestr +=("    WorldInteractable(\n")
            interactablestr +=("      shape: \"" + str(ctype) + "\",\n")
            interactablestr +=("      translation: Vec3(" + str(t[0])+","+str(t[2])+","+str(-t[1]) + "),\n")
            interactablestr +=("      rotation: Quat("+str(r[1])+","+str(r[3])+","+str(-r[2])+","+str(r[0])+","+"),\n")
            interactablestr +=("      scale: Vec3(" + str(abs(s[0]))+","+str(abs(s[2]))+","+str(abs(s[1])) + "),\n")
            interactablestr +=("      interaction: "+interaction+",\n")
            interactablestr +=("    ),\n")

        if obj.name.startswith("Prop"):
            ptype = obj.name.split(".")[1]
            matrix_world = obj.matrix_world
            t = matrix_world.to_translation()
            r = matrix_world.to_quaternion()
            s = matrix_world.to_scale()
            animatable = "None"
            if "animatable" in obj:
                animatable_name = "\""+obj["animatable"]+"\""
                animatable = "Some("+animatable_name+")"

            propstr +=("    WorldProp(\n")
            propstr +=("      prop: \"" + str(ptype) + "\",\n")
            propstr +=("      translation: Vec3(" + str(t[0])+","+str(t[2])+","+str(-t[1]) + "),\n")
            propstr +=("      rotation: Quat("+str(r[1])+","+str(r[3])+","+str(-r[2])+","+str(r[0])+","+"),\n")
            propstr +=("      scale: Vec3(" + str(abs(s[0]))+","+str(abs(s[1]))+","+str(abs(s[2])) + "),\n")
            propstr +=("      animatable: "+animatable+",\n")
            propstr +=("    ),\n")


    ronstr = "WorldAsset(\n"
    ronstr += "  colliders: [\n"
    ronstr += colliderstr
    ronstr += "  ],\n"
    ronstr += "  doors: [\n"
    ronstr += doorstr
    ronstr += "  ],\n"
    ronstr += "  interactables: [\n"
    ronstr += interactablestr
    ronstr += "  ],\n"
    ronstr += "  props: [\n"
    ronstr += propstr
    ronstr += "  ],\n"
    ronstr += ")\n"


    return ronstr

def write_some_data(filepath):
    ronstr = build_ron()
    println(ronstr)
#    fd = os.open(filepath, os.O_RDWR|os.O_CREAT)
#    os.truncate(fd, 0)
#    numBytes = os.write(fd, str.encode(ronstr))
#    os.close(fd)
#    print("Wrote ("+str(numBytes)+"b): "+filepath)

    return {'FINISHED'}

write_some_data(".")