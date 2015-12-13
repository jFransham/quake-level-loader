//================================================
//SKY
//================================================

//BY OBSIDIAN/AEON

textures/phdm5/sky
{
	
	qer_editorimage textures/phdm5/env/miramar_ft.tga
	skyparms textures/phdm5/env/miramar - -   
	q3map_sunExt 1 1 .93 750 323 70.5 3 32
	q3map_lightmapFilterRadius 0 8
	q3map_skyLight 140 6
	surfaceparm sky
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm nodlight
	nopicmip
	nomipmaps	
}

//SUN FLARES
//BY HIPSHOT


textures/phdm5/flare1
{
	surfaceparm nonsolid	
        surfaceparm nomarks	
        surfaceparm nolightmap 
    	surfaceparm trans
        deformVertexes autosprite    	
        {
		clampmap textures/phdm5/flare1.tga
                tcmod rotate 10
                blendFunc add
                rgbGen identity
	}
}

textures/phdm5/flare2
{
	surfaceparm nonsolid	
        surfaceparm nomarks	
        surfaceparm nolightmap 
    	surfaceparm trans
        deformVertexes autosprite    	   
        {
		clampmap textures/phdm5/flare2.tga
                tcmod rotate -10
                blendFunc add
                rgbGen identity
	}
}

textures/phdm5/flare3
{
	surfaceparm nonsolid	
        surfaceparm nomarks	
        surfaceparm nolightmap 
    	surfaceparm trans
        deformVertexes autosprite    	 
        {
		clampmap textures/phdm5/flare3.tga
                tcmod rotate 10
                blendFunc add
                rgbGen identity
	}
}

////================================================
////TERRAIN
////================================================

textures/phdm5/alpha_000
{
   qer_editorimage textures/phdm5/editor/alpha_000.tga
   q3map_alphaMod volume
   q3map_alphaMod set 0.0
   surfaceparm nodraw
   surfaceparm nonsolid
   surfaceparm trans
   qer_trans 0.7
}

textures/phdm5/alpha_100
{
   qer_editorimage textures/phdm5/editor/alpha_100.tga
   q3map_alphaMod volume
   q3map_alphaMod set 1.0
   surfaceparm nodraw
   surfaceparm nonsolid
   surfaceparm trans
   qer_trans 0.7
}

textures/phdm5/ter_concrete1
{
   qer_editorimage textures/phdm5/concrete1.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/concrete1.tga
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

textures/phdm5/ter_dirt
{
   qer_editorimage textures/phdm5/ter_dirt.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/ter_dirt.tga
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

textures/phdm5/ter_grass
{
   qer_editorimage textures/phdm5/ter_grass.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/ter_grass.tga
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

textures/phdm5/ter_conc_dirt
{
   qer_editorimage textures/phdm5/editor/ter_conc_dirt.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/concrete1.tga
      rgbGen identity
   }
   {
      map textures/phdm5/ter_dirt.tga
      blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
      alphaGen vertex
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

textures/phdm5/ter_conc_grass
{
   qer_editorimage textures/phdm5/editor/ter_conc_grass.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/concrete1.tga
      rgbGen identity
   }
   {
      map textures/phdm5/ter_grass.tga
      blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
      alphaGen vertex
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

textures/phdm5/ter_dirt_grass
{
   qer_editorimage textures/phdm5/editor/ter_dirt_grass.tga
   q3map_tcGen ivector ( 128 0 0 ) ( 0 128 0 )
   q3map_nonplanar
   q3map_shadeAngle 75
   {
      map textures/phdm5/ter_grass.tga
      rgbGen identity
   }
   {
      map textures/phdm5/ter_dirt.tga
      blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
      alphaGen vertex
      rgbGen identity
   }
   {
      map $lightmap
      blendFunc GL_DST_COLOR GL_ZERO
      rgbGen identity
   }
}

////================================================
////LIQUIDS
////================================================

//BY HIPSHOT

textures/phdm5/water //the visible water surface, fades into nothing when alpha blended, used towards walls
{
	qer_editorimage textures/phdm5/water.tga
	q3map_globaltexture
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	noMipMaps
	cull none	
	{
		map textures/phdm5/water.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen identitylighting
		alphaGen vertex	
		tcmod scale 1 1			
		tcMod turb 0 .15 0 .015	
	}	
	{
		map textures/phdm5/water2.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen identitylighting
		alphaGen vertex	
		tcmod scale 1 1			
		tcMod turb 0 -.15 0 -.015				
	}			
}

//================================================
//ALPHA CHANNELS
//================================================

//"NOTHING" STAGE BY M4XPOWER FOR VERTEX MODE OPTIMIZATION

textures/phdm5/decal_sticker_electric
{
	surfaceparm trans
	surfaceparm nomarks
        nopicmip
	polygonOffset
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
	{
		map textures/phdm5/decal_sticker_electric.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

textures/phdm5/decal_wires
{
	surfaceparm trans
	surfaceparm nomarks		
        nopicmip
	polygonOffset
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
	{
		map textures/phdm5/decal_wires.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

textures/phdm5/decal_paper1a
{
	
	cull disable
	surfaceparm alphashadow
	surfaceparm trans	
   	surfaceparm nonsolid
   	surfaceparm nomarks
        nopicmip
	polygonOffset
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
	{
		map textures/phdm5/decal_paper1a.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

textures/phdm5/decal_paper1b
{
	
	cull disable
	surfaceparm alphashadow
	surfaceparm trans	
   	surfaceparm nonsolid
   	surfaceparm nomarks
        nopicmip
	polygonOffset
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
	{
		map textures/phdm5/decal_paper1b.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}
textures/phdm5/metb_grate
{
//	cull disable
//	surfaceparm alphashadow
	surfaceparm metalsteps
	surfaceparm trans
	nopicmip
        {
                map textures/phdm5/metb_grate.tga
                alphaFunc GE128
		depthWrite
		//rgbGen vertex
        }
        {
		map $lightmap
		rgbGen identity
		blendFunc filter
		depthFunc equal
	}
}

//DECALS
//BY SOCK

textures/phdm5/decal_plant1
{
	qer_editorimage textures/plants_soc/decal_plant1.tga
	q3map_bounceScale 0
   	surfaceparm nonsolid
	surfaceparm trans
   	surfaceparm nomarks
	surfaceparm nolightmap
   	polygonOffset
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
     		map textures/phdm5/decal_plant1.tga
      		blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
      		detail
   	}
}

//BY TABUN

textures/phdm5/tab_decal_leak_b
{
   noPicMip
   polygonOffset
   surfaceparm nonsolid
   surfaceparm nomarks
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   {
      map textures/phdm5/tab_decal_leak_b.tga
      blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
   }
}

textures/phdm5/tab_decal_stain_b
{
   noPicMip
   polygonOffset
   surfaceparm nonsolid
   surfaceparm nomarks
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   {
      map textures/phdm5/tab_decal_stain_b.tga
      blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
   }

}

textures/phdm5/tab_decal_stain_c
{
   noPicMip
   polygonOffset
   surfaceparm nonsolid
   surfaceparm nomarks
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   {
      map textures/phdm5/tab_decal_stain_c.tga
      blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
   }

}

textures/phdm5/tab_decal_stain_d
{
   noPicMip
   polygonOffset
   surfaceparm nonsolid
   surfaceparm nomarks
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   {
      map textures/phdm5/tab_decal_stain_d.tga
      blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
   }
}

textures/phdm5/tab_decal_stain_e
{
   noPicMip
   polygonOffset
   surfaceparm nonsolid
   surfaceparm nomarks
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   {
      map textures/phdm5/tab_decal_stain_e.tga
      blendFunc GL_ZERO GL_ONE_MINUS_SRC_COLOR
   }

}

//WEAPON MARKERS
//BY HIPSHOT

textures/phdm5/wp_sg
{
	qer_editorimage textures/phdm5/wp_sg.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset	
	nopicmip
	{
		map textures/phdm5/wp_sg.jpg
		rgbGen identitylighting
		blendfunc add
	}
}

textures/phdm5/wp_rl
{
	qer_editorimage textures/phdm5/wp_rl.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset
	nopicmip	
	{
		map textures/phdm5/wp_rl.jpg
		rgbGen identitylighting
		blendfunc add
	}
}

textures/phdm5/wp_rg
{
	qer_editorimage textures/phdm5/wp_rg.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset
	nopicmip	
	{
		map textures/phdm5/wp_rg.jpg
		rgbGen identitylighting
		blendfunc add
	}
}

textures/phdm5/wp_lg
{
	qer_editorimage textures/phdm5/wp_lg.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset	
	nopicmip
	{
		map textures/phdm5/wp_lg.jpg
		rgbGen identitylighting
		blendfunc add
	}
}

textures/phdm5/wp_pg
{
	qer_editorimage textures/phdm5/wp_pg.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset
	nopicmip	
	{
		map textures/phdm5/wp_pg.jpg
		rgbGen identitylighting
		blendfunc add
	}
}

textures/phdm5/wp_gl
{
	qer_editorimage textures/phdm5/wp_gl.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm trans
	surfaceparm nonsolid
	sort 6
	polygonOffset	
	nopicmip
	{
		map textures/phdm5/wp_gl.jpg
		rgbGen identitylighting
		blendfunc add
	}
}


//================================================
//METAL STEPS
//================================================

textures/phdm5/metb
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/metb.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/metb_seam
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/metb_seam.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/metb_clang2
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/metb_clang2.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/metb_mach1
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/metb_mach1.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/metr_seam
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/metr_seam.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/pipe1a
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/pipe1a.tga
		blendfunc filter
		rgbGen identity
	}
}

textures/phdm5/pipe1b
{
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/pipe1b.tga
		blendfunc filter
		rgbGen identity
	}
}


//================================================
//PICMIP
//================================================

textures/phdm5/gauge1
{
	nopicmip
	{
		map $lightmap
		rgbGen identity
		tcGen lightmap
	}
	{
		map textures/phdm5/gauge1.tga
		blendfunc filter
		rgbGen identity
	}
}

//================================================
//PHONG SHADING
//================================================

//PHONG BRICK

textures/phdm5/brick1a
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/brick1a.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/brick1a.tga
		blendFunc filter
	}
}

textures/phdm5/brick1b
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/brick1b.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/brick1b.tga
		blendFunc filter
	}
}

textures/phdm5/brick1a_mossy
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/brick1a_mossy.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/brick1a_mossy.tga
		blendFunc filter
	}
}

textures/phdm5/brick1b_mossy
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/brick1b_mossy.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/brick1b_mossy.tga
		blendFunc filter
	}
}

//PHONG METAL

textures/phdm5/metb_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metb.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metb.tga
		blendFunc filter
	}
}

textures/phdm5/metb_seam_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metb_seam.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metb_seam.tga
		blendFunc filter
	}
}

textures/phdm5/metb_mach1_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metb_mach1.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metb_mach1.tga
		blendFunc filter
	}
}

textures/phdm5/metb_grid_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metb_grid.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metb_grid.tga
		blendFunc filter
	}
}

textures/phdm5/metg_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metg.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metg.tga
		blendFunc filter
	}
}

textures/phdm5/metg_seam_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metg_seam.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metg_seam.tga
		blendFunc filter
	}
}


textures/phdm5/metg_mossy_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metg_mossy.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metg_mossy.tga
		blendFunc filter
	}
}

textures/phdm5/metg_mossy_seam_phong
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/metg_mossy_seam.tga
	surfaceparm metalsteps
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/metg_mossy_seam.tga
		blendFunc filter
	}
}

//PHONG WOOD

textures/phdm5/wood1_plank
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/wood1_plank.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/wood1_plank.tga
		blendFunc filter
	}
}

textures/phdm5/wood2_plank
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/wood2_plank.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/wood2_plank.tga
		blendFunc filter
	}
}

textures/phdm5/wood3_board
{
	q3map_nonplanar
	q3map_shadeangle 120
	surfaceparm nomarks
	qer_editorimage textures/phdm5/wood3_board.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/wood3_board.tga
		blendFunc filter
	}
}

//PHONG CONCRETE

textures/phdm5/concrete1
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/concrete1.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/concrete1.tga
		blendFunc filter
	}
}

textures/phdm5/concrete1_steps
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/concrete1_steps.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/concrete1_steps.tga
		blendFunc filter
	}
}

textures/phdm5/concrete3_tile
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/concrete3_tile.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/concrete3_tile.tga
		blendFunc filter
	}
}

textures/phdm5/concrete4_mossy
{
	q3map_nonplanar
	q3map_shadeangle 120
	qer_editorimage textures/phdm5/concrete4_mossy.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/concrete4_mossy.tga
		blendFunc filter
	}
}

textures/phdm5/concrete4_trim_mossy
{
	q3map_nonplanar
	q3map_shadeangle 179
	qer_editorimage textures/phdm5/concrete4_trim_mossy.tga
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/concrete4_trim_mossy.tga
		blendFunc filter
	}
}

//================================================
//GLASS
//================================================

//BY FKD

textures/phdm5/glass
{
        qer_editorimage textures/phdm5/glass.tga
        qer_trans 0.50
        surfaceparm nolightmap
	surfaceparm nodlight
        surfaceparm trans
        {
                map textures/phdm5/glass.tga
                blendfunc filter
		rgbGen identity
        }
        {
                map textures/phdm5/glass_env.tga
                blendfunc add
                rgbGen identity
                tcGen environment
        }
}

//BY SST13

textures/phdm5/glass_break_a
{
	qer_editorimage textures/phdm5/glass_break_a.tga
	qer_trans 0.9
	surfaceparm nolightmap
	surfaceparm nodlight
	surfaceparm trans
	sort additive
	{
		map textures/phdm5/glass_break_a.tga
		blendfunc filter
		rgbGen identity
		depthWrite
		alphaFunc GE128
	}
	{
		map textures/phdm5/glass_env.tga
		blendfunc add
		rgbGen identity
		tcGen environment
		depthFunc equal
	}
}

textures/phdm5/glass_break_b
{
	qer_editorimage textures/phdm5/glass_break_b.tga
	qer_trans 0.9
	surfaceparm nolightmap
	surfaceparm nodlight
	surfaceparm trans
	sort additive
	{
		map textures/phdm5/glass_break_b.tga
		blendfunc filter
		rgbGen identity
		depthWrite
		alphaFunc GE128
	}
	{
		map textures/phdm5/glass_env.tga
		blendfunc add
		rgbGen identity
		tcGen environment
		depthFunc equal
	}
}

textures/phdm5/glass_break_c
{
	qer_editorimage textures/phdm5/glass_break_c.tga
	qer_trans 0.9
	surfaceparm nolightmap
	surfaceparm nodlight
	surfaceparm trans
	sort additive
	{
		map textures/phdm5/glass_break_c.tga
		blendfunc filter
		rgbGen identity
		depthWrite
		alphaFunc GE128
	}
	{
		map textures/phdm5/glass_env.tga
		blendfunc add
		rgbGen identity
		tcGen environment
		depthFunc equal
	}
}

textures/phdm5/glass_break_d
{
	qer_editorimage textures/phdm5/glass_break_d.tga
	qer_trans 0.9
	surfaceparm nolightmap
	surfaceparm nodlight
	surfaceparm trans
	sort additive
	{
		map textures/phdm5/glass_break_d.tga
		blendfunc filter
		rgbGen identity
		depthWrite
		alphaFunc GE128
	}
	{
		map textures/phdm5/glass_env.tga
		blendfunc add
		rgbGen identity
		tcGen environment
		depthFunc equal
	}
}

//================================================
//PLANTS
//================================================

//LEAVES
//BY SOCK

textures/phdm5/plant_leaf01a
{
	qer_editorimage textures/phdm5/plant_leaf01a.tga
	q3map_cloneShader textures/phdm5/plant_leaf01a_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_alphafunc greater 0.5
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf01a_back
{
	qer_editorimage textures/phdm5/plant_leaf01a.tga

	q3map_invert
	q3map_vertexScale 3.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}


textures/phdm5/plant_leaf01b
{
	qer_editorimage textures/phdm5/plant_leaf01b.tga
	q3map_cloneShader textures/phdm5/plant_leaf01b_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_alphafunc greater 0.5
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01b.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01b.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf01b_back
{
	qer_editorimage textures/phdm5/plant_leaf01b.tga

	q3map_invert
	q3map_vertexScale 3.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01b.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01b.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}


textures/phdm5/plant_leaf01d
{
	qer_editorimage textures/phdm5/plant_leaf01d.tga
	q3map_cloneShader textures/phdm5/plant_leaf01d_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_alphafunc greater 0.5
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01d.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01d.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf01d_back
{
	qer_editorimage textures/phdm5/plant_leaf01d.tga

	q3map_invert
	q3map_vertexScale 3.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf01d.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf01d.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf02a
{
	qer_editorimage textures/phdm5/plant_leaf02a.tga
	q3map_cloneShader textures/phdm5/plant_leaf02a_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_alphafunc greater 0.5
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf02a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf02a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf02a_back
{
	qer_editorimage textures/phdm5/plant_leaf02a.tga

	q3map_invert
	q3map_vertexScale 3.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf02a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf02a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf05a
{
	qer_editorimage textures/phdm5/plant_leaf05a.tga
	q3map_cloneShader textures/phdm5/plant_leaf05a_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_alphafunc greater 0.5
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf05a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf05a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_leaf05a_back
{
	qer_editorimage textures/phdm5/plant_leaf05a.tga

	q3map_invert
	q3map_vertexScale 3.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_leaf05a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_leaf05a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

//GRASS
//BY SOCK

textures/phdm5/plant_grass01a
{
	qer_editorimage textures/phdm5/plant_grass01a.tga
	q3map_cloneShader textures/phdm5/plant_grass01a_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm pointlight
	qer_trans 0.99
	nopicmip

	deformVertexes wave 16 sin 0 0.5 0 0.1
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_grass01a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_grass01a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_grass01a_back
{
	qer_editorimage textures/phdm5/plant_grass01a.tga

	q3map_invert
	q3map_vertexScale 1.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm pointlight
	nopicmip

	deformVertexes wave 16 sin 0 0.5 0 0.1
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_grass01a.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_grass01a.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_grass01d
{
	qer_editorimage textures/phdm5/plant_grass01d.tga
	q3map_cloneShader textures/phdm5/plant_grass01d_back

	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm pointlight
	qer_trans 0.99
	nopicmip

	deformVertexes wave 16 sin 0 0.5 0 0.1
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_grass01d.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_grass01d.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_grass01d_back
{
	qer_editorimage textures/phdm5/plant_grass01d.tga

	q3map_invert
	q3map_vertexScale 1.5
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm pointlight
	nopicmip

	deformVertexes wave 16 sin 0 0.5 0 0.1
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_grass01d.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_grass01d.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

//MUSHROOM
//BY SOCK
textures/phdm5/plant_mush_top01
{
	qer_editorimage textures/phdm5/plant_mush_top01.tga
	q3map_cloneShader textures/phdm5/plant_mush_back01
	q3map_vertexScale 1.25
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	surfaceparm alphashadow
	qer_trans 0.99
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_mush_top01.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_mush_top01.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_mush_back01
{
	qer_editorimage textures/phdm5/plant_mush_back01.tga

	q3map_invert
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	surfaceparm nolightmap
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
   	{
		map textures/phdm5/plant_mush_back01.tga
		blendFunc GL_SRC_ALPHA GL_ONE_MINUS_SRC_ALPHA
		rgbGen vertex
	}
	{
		map textures/phdm5/plant_mush_back01.tga
		alphaFunc GE128
		rgbGen vertex
		depthWrite
	}
}

textures/phdm5/plant_mush_stem01
{
	qer_editorimage textures/phdm5/plant_mush_stem01.tga
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	nopicmip

	q3map_nonplanar
	q3map_shadeAngle 75
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	}
	{
		map $lightmap
		rgbGen identity
	}
	{
		map textures/phdm5/plant_mush_stem01.tga
		blendFunc GL_DST_COLOR GL_ZERO
		tcmod Scale 2 2
	}
}

//WATER LILIES
//BY HIPSHOT

textures/phdm5/plant_lily1
{
	qer_alphafunc greater 0.5
//	surfaceparm alphashadow
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	qer_trans 0.99
//	deformVertexes bulge 128 1 2	
	polygonOffset	
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	} 
	{
		map textures/phdm5/plant_lily1.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}		
}

textures/phdm5/plant_lily2
{
	qer_alphafunc greater 0.5
//	surfaceparm alphashadow
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	qer_trans 0.99
//	deformVertexes bulge 128 1 2	
	polygonOffset	
	nopicmip
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	} 
	{
		map textures/phdm5/plant_lily2.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}		
}

//IVY
//BY HIPSHOT

textures/phdm5/plant_ivy1
{
	qer_editorimage textures/phdm5/plant_ivy1.tga
	qer_alphafunc greater 0.5
	q3map_shadeangle 179
	surfaceparm alphashadow
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nomarks
	qer_trans 0.99
   	noPicMip	
	cull none
	{
		map textures/phdm5/nothing.tga   
		blendfunc blend
		tcmod scale 0 0
		depthFunc equal
	} 	
	{
		map textures/phdm5/plant_ivy1.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}		
}

//TREES
//BY HIPSHOT

textures/phdm5/leaf
{
	qer_editorimage textures/phdm5/leaf.tga
	qer_alphafunc greater 0.5
	qer_trans 0.99	
	surfaceparm alphashadow
	surfaceparm trans
	surfaceparm nomarks
	surfaceparm nonsolid				
	nopicmip
	cull none	     
	deformVertexes wave 25 sin 3 8 .1 0.1    //deformVertexes wave 25 sin 3 2 .1 0.1      
	{
		map textures/phdm5/leaf.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		rgbGen const ( 1 1 1 )		
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

textures/phdm5/trunk
{
	qer_editorimage textures/phdm5/trunk.tga	
	q3map_nonplanar
	q3map_shadeangle 180
	nopicmip
	{
		map textures/phdm5/trunk.tga
		rgbGen identity
	}
	{
		map $lightmap
		blendFunc GL_DST_COLOR GL_ZERO
		rgbGen identity
	}
}

//================================================
//FAN
//================================================

textures/phdm5/fan
{
	surfaceparm nomarks
	surfaceparm trans	
	nopicmip
	{
		map textures/phdm5/fan.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

//BY HIPSHOT

textures/phdm5/fan_frame
{
	surfaceparm nomarks
	surfaceparm alphashadow
	nopicmip
	{
		map textures/phdm5/fan_frame.tga
		blendFunc GL_ONE GL_ZERO
		alphaFunc GE128
		depthWrite
		rgbGen identity
	}
	{
		map $lightmap
		rgbGen identity
		blendFunc GL_DST_COLOR GL_ZERO
		depthFunc equal
	}
}

//================================================
//SFX
//================================================

//JUMP PAD
//BY HIPSHOT

textures/phdm5/jumpgrid
{
	qer_editorimage textures/phdm5/jumpgrid.jpg
	//q3map_lightimage textures/colors/white.jpg
	surfaceparm trans
	surfaceparm nonsolid	
	surfaceparm nomarks
	surfaceparm nodamage
	surfaceparm nolightmap
	q3map_surfacelight 1500
	nopicmip
	{
		map textures/phdm5/jumpgrid.tga
		rgbGen identityLighting
	}
	{
		clampmap textures/phdm5/jumpsquare.jpg //timad med moave square, den stora.
		blendfunc add
		tcMod stretch sawtooth .1 7 .1 1.3 //size den kommer från; hur mycket den växer; tid till spawn; hastighet.
		//rgbGen wave inversesawtooth 0 1 .1 1.3 //fada bort			
	}
	{
		map textures/phdm5/jumpgrid.tga
		blendfunc blend
		rgbGen identityLighting
	}	
}

textures/phdm5/jumpring1
{
	qer_editorimage textures/phdm5/jumpring1.jpg
	qer_trans .5	
	surfaceparm noimpact
	surfaceparm nolightmap
	cull none
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nodlight
	deformvertexes move 0 0 50 sawtooth 0 1 1 1.3
	nopicmip
	{
		clampmap textures/phdm5/jumpring1.jpg
		rgbGen wave sawtooth 1 -1 1 1.3
		blendfunc add
	}
}

textures/phdm5/jumpring2
{
	qer_editorimage textures/phdm5/jumpring2.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	cull none
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nodlight
	deformvertexes move 0 0 50 sawtooth 0 1 .875 1.3
	nopicmip
	{
		clampmap textures/phdm5/jumpring2.jpg
		rgbGen wave sawtooth 1 -1 .875 1.3
		blendfunc add
	}
}

textures/phdm5/jumpring3
{
	qer_editorimage textures/phdm5/jumpring3.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	cull none
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nodlight
	deformvertexes move 0 0 50 sawtooth 0 1 .75 1.3
	nopicmip
	{
		clampmap textures/phdm5/jumpring3.jpg
		rgbGen wave sawtooth 1 -1 .75 1.3
		blendfunc add
	}
}

textures/phdm5/jumpring4
{
	qer_editorimage textures/phdm5/jumpring4.jpg
	qer_trans .5		
	surfaceparm noimpact
	surfaceparm nolightmap
	cull none
	surfaceparm trans
	surfaceparm nonsolid
	surfaceparm nodlight
	deformvertexes move 0 0 50 sawtooth 0 1 .625 1.3
	nopicmip
	{
		clampmap textures/phdm5/jumpring4.jpg
		rgbGen wave sawtooth 1 -1 .625 1.3
		blendfunc add
	}
}

//TELE
//BY HIPSHOT

textures/phdm5/tele
{
	qer_editorimage textures/phdm5/tele.jpg
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm nonsolid
	surfaceparm nomarks
	q3map_surfacelight 50
	q3map_backSplash 10 25	
	nopicmip
	{
		map textures/phdm5/tele.jpg
		rgbGen identityLighting	
	}	
	{
		map textures/phdm5/tele.jpg
		blendfunc add	
		rgbGen identityLighting
		rgbGen wave sin .1 .2 0 .2
	}
}

textures/phdm5/teleglow
{
	qer_editorimage textures/phdm5/teleglow.jpg
	surfaceparm trans
	surfaceparm noimpact
	surfaceparm nolightmap
	surfaceparm nonsolid	
	surfaceparm nomarks
//	cull disable
	nopicmip
	{
		map textures/phdm5/teleglow.jpg
		blendfunc add
		rgbGen identity
		rgbGen wave sin 1 .2 0 .2		
	}
}