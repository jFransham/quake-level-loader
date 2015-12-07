bitflags! {
    flags ContentFlags: u32 {
        const SOLID3 =        0x00000001,   // an eye is never valid in a solid
        const LAVA =          0x00000008,
        const SLIME =         0x00000010,
        const WATER =         0x00000100,
        const FOG =           0x00001000,

        const NOTTEAM1 =      0x00000080,
        const NOTTEAM2 =      0x00000100,
        const NOBOTCLIP =     0x00000200,

        const AREAPORTAL =    0x00008000,

        const PLAYERCLIP =    0x00010000,
        const MONSTERCLIP =   0x00020000,
        //bot specific contents types
        const TELEPORTER =    0x00040000,
        const JUMPPAD =       0x00080000,
        const CLUSTERPORTAL = 0x00100000,
        const DONOTENTER =    0x00200000,
        const BOTCLIP =       0x00400000,
        const MOVER =         0x00800000,

        const ORIGIN =        0x01000000,   // removed before bsping an entity

        const BODY =          0x02000000,   // should never be on a brush, only in game
        const CORPSE =        0x04000000,
        const DETAIL =        0x08000000,   // brushes not used for the bsp
        const STRUCTURAL =    0x10000000,   // brushes used for the bsp
        const TRANSLUCENT =   0x20000000,   // don't consume surface fragments inside
        const TRIGGER =       0x40000000,
        const NODROP =        0x80000000,   // don't leave bodies or items (death fog, lava)
    }
}

bitflags! {
    flags SurfaceFlags: u32 {
        const NODAMAGE =      0x00001, // never give falling damage
        const SLICK =         0x00002, // effects game physics
        const SKY =           0x00004, // lighting from environment map
        const LADDER =        0x00008,
        const NOIMPACT =      0x00010, // don't make missile explosions
        const NOMARKS =       0x00020, // don't leave missile marks
        const FLESH =         0x00040, // make flesh sounds and effects
        const NODRAW =        0x00080, // don't generate a drawsurface at all
        const HINT =          0x00100, // make a primary bsp splitter
        const SKIP =          0x00200, // completely ignore, allowing non-closed brushes
        const NOLIGHTMAP =    0x00400, // surface doesn't need a lightmap
        const POINTLIGHT =    0x00800, // generate lighting info at vertexes
        const METALSTEPS =    0x01000, // clanking footsteps
        const NOSTEPS =       0x02000, // no footstep sounds
        const NONSOLID =      0x04000, // don't collide against curves with this set
        const LIGHTFILTER =   0x08000, // act as a light filter during q3map -light
        const ALPHASHADOW =   0x10000, // do per-pixel light shadow casting in q3map
        const NODLIGHT =      0x20000, // don't dlight even if solid (solid lava, skies)
        const DUST =          0x40000, // leave a dust trail when walking on this surface
    }
}
