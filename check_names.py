import json

with open('rust_functions_manifest.json') as f:
    data = json.load(f)

# Search for specific functions
targets = ['count_sunflowers', 'initialize', 'check_3d', 'poly_fill_3d', 
           'PoolEffectInitialize', 'CountSunFlowers', 'Check3D', 'PolyFill3D',
           'format', 'skip_input_data', 'glad_gl_get_extensions',
           'Ref', 'Rand', 'NormalDrawLine', 'NormalDrawLineAA',
           'SWDrawTriangle', 'Tod_SWTri_AddAllDrawTriFuncs', 'TodBltMatrix',
           'BltMatrixHelper', 'BltTrianglesTexHelper',
           'Mix_ReserveChannels', 'Mix_FadeInMusic', 'Mix_SetMusicPosition',
           'StrFormat']

for entry in data:
    name = entry.get('name', '')
    parent = entry.get('parent', '')
    if name in targets or parent + '::' + name in targets:
        sig = entry.get('signature', '')[:100]
        print(f"{parent}::{name} (kind={entry.get('kind','')}) sig={sig}")
