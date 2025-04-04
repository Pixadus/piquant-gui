// Copyright (c) 2018-2022 California Institute of Technology (“Caltech”) and
// University of Washington. U.S. Government sponsorship acknowledged.
// All rights reserved.
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
// * Neither the name of Caltech nor its operating division, the Jet Propulsion
//   Laboratory, nor the names of its contributors may be used to endorse or
//   promote products derived from this software without specific prior written
//   permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#include "fpEdgeAbsorption.h"

using namespace std;

void fpEdgeAbsorption  ( const XrayEdge edgeIn, const XrayXsectTable &absTables,
		const vector <float> &energyIn, vector <float> &edgeAbs ) {
// calculates subshell photoelectric x-ray absorption of given element at specified energies
//		returns vector edgeAbs
//	assumes list of energies is ordered highest to lowest
//     Copyright 2001  W. T. Elam

//		calculate cross section at specified energy as total photoelectric absorption
//			divided by jump ratio for each edge between edge of interest and specified energy
//		this gives only the absorption of the subshell which produces the given absorption edge

//		calculate total photoelectric absorption at each energy
	edgeAbs.resize(energyIn.size());
	int i;
	for ( i=0; i<energyIn.size(); i++ ) {
		edgeAbs[i] = absTables.photo(energyIn[i]);
	};

//		get list of edges with energy less than highest input energy and
//			higher than edge energy of input edge
	float ee = edgeIn.energy();
	vector <EdgeIndex> edgeIndexList;
	XrayEdge::numberOfEdges ( edgeIndexList, edgeIn.element(),
			energyIn[0] );
	if ( edgeIndexList.size() <= 0 ) return;
	vector <float> edgeEnergy(0);
	vector <float> edgeJump(0);
	int edgeIndex;
	for ( edgeIndex=0; edgeIndex<edgeIndexList.size(); edgeIndex++ ) {
		XrayEdge thisEdge ( edgeIn.element(), edgeIndexList[edgeIndex] );
		if ( thisEdge.energy() > ee ) {
			edgeEnergy.push_back ( thisEdge.energy() );
			edgeJump.push_back ( thisEdge.jump() );
		};
	};
	if ( edgeEnergy.size() <= 0 ) return;
	for ( i=0; i<energyIn.size(); i++ ) {
		for ( edgeIndex=0; edgeIndex<edgeEnergy.size(); edgeIndex++ ) {
			if ( edgeEnergy[edgeIndex] < energyIn[i] ) {
				edgeAbs[i] /= edgeJump[edgeIndex];
			};
		};
	};
};
